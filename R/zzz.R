class(ExternalWindowController) <- NULL
`ExternalWindowController`$`new` <- function(server =  system.file("winit_r_package_server.exe", package = "winitRpackage")) {
  .savvy_wrap_ExternalWindowController(.Call(savvy_ExternalWindowController_new__impl, server))
}
class(ExternalWindowController) <- "ExternalWindowController__bundle"