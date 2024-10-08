
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

SEXP savvy_foo__impl(void) {
    SEXP res = savvy_foo__ffi();
    return handle_result(res);
}

SEXP savvy_foo2__impl(void) {
    SEXP res = savvy_foo2__ffi();
    return handle_result(res);
}

SEXP savvy_AppController_new__impl(void) {
    SEXP res = savvy_AppController_new__ffi();
    return handle_result(res);
}

SEXP savvy_AppController_resize__impl(SEXP self__, SEXP c_arg__width, SEXP c_arg__height) {
    SEXP res = savvy_AppController_resize__ffi(self__, c_arg__width, c_arg__height);
    return handle_result(res);
}

SEXP savvy_AppController_close__impl(SEXP self__) {
    SEXP res = savvy_AppController_close__ffi(self__);
    return handle_result(res);
}



static const R_CallMethodDef CallEntries[] = {
    {"savvy_foo__impl", (DL_FUNC) &savvy_foo__impl, 0},
    {"savvy_foo2__impl", (DL_FUNC) &savvy_foo2__impl, 0},
    {"savvy_AppController_new__impl", (DL_FUNC) &savvy_AppController_new__impl, 0},
    {"savvy_AppController_resize__impl", (DL_FUNC) &savvy_AppController_resize__impl, 3},
    {"savvy_AppController_close__impl", (DL_FUNC) &savvy_AppController_close__impl, 1},

    {NULL, NULL, 0}
};

void R_init_winitPumpRPackage(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.

}
