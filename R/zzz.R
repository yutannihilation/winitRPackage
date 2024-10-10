pkg_cache_dir <- function() {
  normalizePath(tools::R_user_dir("winitRpackage", "cache"), mustWork = FALSE)
}

server_path <- function() {
  bin <- if (Sys.info()[["sysname"]] == "Windows") {
    "winit_r_package_server.exe"
  } else {
    "winit_r_package_server"
  }

  file.path(pkg_cache_dir(), bin)

  stop("TODO")
}

# overwrite
class(ExternalWindowController) <- NULL
`ExternalWindowController`$`new` <- function(server = server_path()) {
  .savvy_wrap_ExternalWindowController(.Call(savvy_ExternalWindowController_new__impl, server))
}
class(ExternalWindowController) <- "ExternalWindowController__bundle"