# overwrite
class(ExternalWindowController) <- NULL
`ExternalWindowController`$`new` <- function(server = server_path()) {
  if (!file.exists(server)) {
    stop("Please run download_server() first")
  }
  
  .savvy_wrap_ExternalWindowController(.Call(savvy_ExternalWindowController_new__impl, server))
}
class(ExternalWindowController) <- "ExternalWindowController__bundle"