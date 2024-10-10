SEXP savvy_run_event_loop_on_spawned_thread__ffi(void);

// methods and associated functions for ExternalWindowController
SEXP savvy_ExternalWindowController_new__ffi(void);
SEXP savvy_ExternalWindowController_open_window__ffi(SEXP self__, SEXP c_arg__title);
SEXP savvy_ExternalWindowController_close_window__ffi(SEXP self__);

// methods and associated functions for SpawnedWindowController
SEXP savvy_SpawnedWindowController_new__ffi(void);
SEXP savvy_SpawnedWindowController_open_window__ffi(SEXP self__, SEXP c_arg__title);
SEXP savvy_SpawnedWindowController_close_window__ffi(SEXP self__);