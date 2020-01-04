//
// Created by david on 2019/12/27.
//

#ifndef C_API_H
#define C_API_H

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#include <setjmp.h>
extern sigjmp_buf *g_exception_stack;
extern void print_from_c(const char* str);
extern void longjmp_routine();

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif // C_API_H
