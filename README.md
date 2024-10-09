An experiment of using winit form R
===================================

The main difficulty is that winit requires to be executed on the main thread, whereas R REPL of course needs the main thread. So, how can we use winit?

1. [`with_any_thread()`](https://docs.rs/winit/latest/winit/platform/wayland/trait.EventLoopBuilderExtWayland.html#tymethod.with_any_thread): available on Linux and Windows
2. fork by `parallel::mcparallel()`: available on Linux and macOS
