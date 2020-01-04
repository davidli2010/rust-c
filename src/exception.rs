use crate::bindings::sigjmp_buf;
use std::mem::MaybeUninit;
use std::os::raw::c_int;
use std::panic::PanicInfo;
use std::sync::atomic::{compiler_fence, Ordering};

#[repr(C)]
pub struct ExceptionStack(MaybeUninit<sigjmp_buf>);

impl ExceptionStack {
    pub fn new() -> Self {
        Self(MaybeUninit::uninit())
    }

    pub fn as_mut_ptr(&mut self) -> *mut ExceptionStack {
        self as *mut ExceptionStack
    }
}

pub struct Exception;

impl Exception {
    pub fn current_exception_stack() -> *mut ExceptionStack {
        unsafe { crate::bindings::g_exception_stack as *mut ExceptionStack }
    }

    pub fn set_exception_stack(stack: *mut ExceptionStack) {
        unsafe {
            crate::bindings::g_exception_stack = stack as *mut sigjmp_buf;
        };
    }
}

#[cfg(target_os = "linux")]
macro_rules! sigsetjmp {
    ($stack: expr, $val: expr) => {
        unsafe { crate::bindings::__sigsetjmp($stack as *mut _, $val) }
    };
}

#[cfg(target_os = "macos")]
macro_rules! sigsetjmp {
    ($stack: expr, $val: expr) => {
        unsafe { crate::bindings::sigsetjmp($stack as *mut _, $val) }
    };
}

#[cfg(unix)]
fn siglongjump(stack: *mut ExceptionStack, value: c_int) {
    unsafe {
        crate::bindings::siglongjmp(stack as *mut _, value);
    }
}

struct JumpContext {
    jump_value: c_int,
}

/// Provides a barrier between Rust and C's usage of the C set/longjmp
///
/// In the case of a longjmp being caught, this will convert that to a panic. For this to work
///   properly, there must be a Rust panic handler (see crate::exception_handler).
pub fn catch_exception<F, R>(func: F) -> R
where
    F: FnOnce() -> R,
{
    let save_exception_stack = Exception::current_exception_stack();

    let mut local_exception_stack = ExceptionStack::new();

    let jumped = sigsetjmp!(local_exception_stack.as_mut_ptr(), 1);
    if jumped != 0 {
        Exception::set_exception_stack(save_exception_stack);

        compiler_fence(Ordering::SeqCst);
        panic!(JumpContext { jump_value: jumped })
    }

    Exception::set_exception_stack(local_exception_stack.as_mut_ptr());

    compiler_fence(Ordering::SeqCst);
    let result = func();
    compiler_fence(Ordering::SeqCst);

    Exception::set_exception_stack(save_exception_stack);

    result
}

/// Exception handler that will catch the `longjmp`.
pub fn exception_handler() -> Box<dyn Fn(&PanicInfo<'_>) + 'static + Sync + Send> {
    Box::new(|info| {
        // downcast info, check if it's the value we need.
        //   this must check if the panic was due to a longjmp
        //   the fence is to make sure the longjmp is not reodered.
        compiler_fence(Ordering::SeqCst);
        if let Some(context) = info.payload().downcast_ref::<JumpContext>() {
            std::rt::update_panic_count(-1);
            // WARNING: do not set this level above Notice (ERROR, FATAL, PANIC), as it will calse
            //   the following longjmp to execute.
            println!("continuing longjmp: {}", info);

            siglongjump(Exception::current_exception_stack(), context.jump_value);
        } else {
            // error level will cause a longjmp in Rust
            println!("panic in Rust: {}", info);
            siglongjump(Exception::current_exception_stack(), 1);
        }

        unreachable!("all above statements should have cause a longjmp to C");
    })
}
