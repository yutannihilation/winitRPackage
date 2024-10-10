An experiment of using winit form R
===================================

The main difficulty is that winit requires to be executed on the main thread, whereas R REPL of course needs the main thread. So, how can we use winit?

1. [`with_any_thread()`](https://docs.rs/winit/latest/winit/platform/wayland/trait.EventLoopBuilderExtWayland.html#tymethod.with_any_thread): available on Linux and Windows.
2. Run an external server process. This is probably slow, but macOS has no other choice but this. They say [XPC](https://developer.apple.com/documentation/xpc) is better in performance, but I haven't tried this yet (mainly because I don't have a macOS machine now).

### Use a spawned process

```r
x <- SpawnedWindowController$new()
x$open_window("foo")
x$close_window()
```

### Use an external process

```r
x <- ExternalWindowController$new()
x$open_window("foo")
x$close_window()
```