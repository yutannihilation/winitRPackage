
#include <stdint.h>
#include <Rinternals.h>
#include <R_ext/Parse.h>

#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_errorcall() directly.
            Rf_errorcall(R_NilValue, "%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}


SEXP savvy_ExternalWindowController_new__impl(void) {
    SEXP res = savvy_ExternalWindowController_new__ffi();
    return handle_result(res);
}

SEXP savvy_ExternalWindowController_new_debug__impl(void) {
    SEXP res = savvy_ExternalWindowController_new_debug__ffi();
    return handle_result(res);
}

SEXP savvy_ExternalWindowController_open_window__impl(SEXP self__, SEXP c_arg__title) {
    SEXP res = savvy_ExternalWindowController_open_window__ffi(self__, c_arg__title);
    return handle_result(res);
}

SEXP savvy_ExternalWindowController_get_window_size__impl(SEXP self__) {
    SEXP res = savvy_ExternalWindowController_get_window_size__ffi(self__);
    return handle_result(res);
}

SEXP savvy_ExternalWindowController_close_window__impl(SEXP self__) {
    SEXP res = savvy_ExternalWindowController_close_window__ffi(self__);
    return handle_result(res);
}

SEXP savvy_SpawnedWindowController_new__impl(void) {
    SEXP res = savvy_SpawnedWindowController_new__ffi();
    return handle_result(res);
}

SEXP savvy_SpawnedWindowController_open_window__impl(SEXP self__, SEXP c_arg__title) {
    SEXP res = savvy_SpawnedWindowController_open_window__ffi(self__, c_arg__title);
    return handle_result(res);
}

SEXP savvy_SpawnedWindowController_get_window_size__impl(SEXP self__) {
    SEXP res = savvy_SpawnedWindowController_get_window_size__ffi(self__);
    return handle_result(res);
}

SEXP savvy_SpawnedWindowController_close_window__impl(SEXP self__) {
    SEXP res = savvy_SpawnedWindowController_close_window__ffi(self__);
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {

    {"savvy_ExternalWindowController_new__impl", (DL_FUNC) &savvy_ExternalWindowController_new__impl, 0},
    {"savvy_ExternalWindowController_new_debug__impl", (DL_FUNC) &savvy_ExternalWindowController_new_debug__impl, 0},
    {"savvy_ExternalWindowController_open_window__impl", (DL_FUNC) &savvy_ExternalWindowController_open_window__impl, 2},
    {"savvy_ExternalWindowController_get_window_size__impl", (DL_FUNC) &savvy_ExternalWindowController_get_window_size__impl, 1},
    {"savvy_ExternalWindowController_close_window__impl", (DL_FUNC) &savvy_ExternalWindowController_close_window__impl, 1},
    {"savvy_SpawnedWindowController_new__impl", (DL_FUNC) &savvy_SpawnedWindowController_new__impl, 0},
    {"savvy_SpawnedWindowController_open_window__impl", (DL_FUNC) &savvy_SpawnedWindowController_open_window__impl, 2},
    {"savvy_SpawnedWindowController_get_window_size__impl", (DL_FUNC) &savvy_SpawnedWindowController_get_window_size__impl, 1},
    {"savvy_SpawnedWindowController_close_window__impl", (DL_FUNC) &savvy_SpawnedWindowController_close_window__impl, 1},
    {NULL, NULL, 0}
};

void R_init_winitRpackage(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.

}
