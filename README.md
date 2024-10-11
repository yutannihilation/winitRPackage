How can we use [winit](https://docs.rs/winit/latest/winit/) from R?
============================

<!-- badges: start -->
[![R-CMD-check](https://github.com/yutannihilation/winitPumpRPackage/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/yutannihilation/winitPumpRPackage/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

The winit Rust crate is a cross-platform library about creating and managing window. If you want to create a GUI with Rust, winit is what you will probably rely on directly or indirectly. For example, [Tauri](https://v2.tauri.app/), which recently released v2, uses a forked-version of winit.

This R package is to show how winit can be used within an R package. The main difficulty is that winit requires to be executed on the main thread, whereas R REPL of course needs to be the main thread. So, how can we solve this conflict?

1. [`with_any_thread()`](https://docs.rs/winit/latest/winit/platform/wayland/trait.EventLoopBuilderExtWayland.html#tymethod.with_any_thread): available on Linux and Windows.
2. Run an external server process. This is probably slow, but macOS has no other choice but this. They say [XPC](https://developer.apple.com/documentation/xpc) is better in performance, but I haven't tried this yet (mainly because I don't have a macOS machine now).

## Installation

This package can be installed from R-universe. You don't need to have Rust installed in your computer!

```r
install.packages("winitRPackage",
  repos = c('https://yutannihilation.r-universe.dev', 'https://cloud.r-project.org')
)
```

## Usages

First, please run `download_server()` to download the server binary.
This will be used by `ExternalWindowController`.

```r
library(winitRPackage)

download_server()
```

### Use an external process

```r
x <- ExternalWindowController$new()

# create a new window titled "foo"
x$open_window("foo")

# get the window size
x$get_window_size()
#> [1] 800 600

# close the window
x$close_window()
```

### Use a spawned process

> [!NOTE]
> For a technical limitation, this doesn't work on macOS!

```r
x <- SpawnedWindowController$new()

# create a new window titled "foo"
x$open_window("foo")

# get the window size
x$get_window_size()
#> [1] 800 600

# close the window
x$close_window()
```
