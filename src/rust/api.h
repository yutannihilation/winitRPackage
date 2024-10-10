

// methods and associated functions for ExternalWindowController
SEXP savvy_ExternalWindowController_new__ffi(SEXP c_arg__server);
SEXP savvy_ExternalWindowController_new_debug__ffi(void);
SEXP savvy_ExternalWindowController_open_window__ffi(SEXP self__, SEXP c_arg__title);
SEXP savvy_ExternalWindowController_get_window_size__ffi(SEXP self__);
SEXP savvy_ExternalWindowController_close_window__ffi(SEXP self__);

// methods and associated functions for SpawnedWindowController
SEXP savvy_SpawnedWindowController_new__ffi(void);
SEXP savvy_SpawnedWindowController_open_window__ffi(SEXP self__, SEXP c_arg__title);
SEXP savvy_SpawnedWindowController_get_window_size__ffi(SEXP self__);
SEXP savvy_SpawnedWindowController_close_window__ffi(SEXP self__);