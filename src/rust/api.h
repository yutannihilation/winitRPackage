SEXP savvy_run_event_loop_on_main_thread__ffi(void);
SEXP savvy_run_event_loop_on_spawned_thread__ffi(void);

// methods and associated functions for WindowController
SEXP savvy_WindowController_new__ffi(void);
SEXP savvy_WindowController_open_window__ffi(SEXP self__, SEXP c_arg__title);
SEXP savvy_WindowController_close_window__ffi(SEXP self__);