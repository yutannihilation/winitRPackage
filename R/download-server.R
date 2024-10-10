pkg_cache_dir <- function() {
  normalizePath(tools::R_user_dir("winitRPackage", "cache"), mustWork = FALSE)
}

server_path <- function() {
  bin <- if (Sys.info()[["sysname"]] == "Windows") {
    "winit_r_package_server.exe"
  } else {
    "winit_r_package_server"
  }

  path <- file.path(pkg_cache_dir(), bin)
}

URL_BASE <- "https://github.com/yutannihilation/winitRPackage/releases/download"

get_latest_release <- function() {
  jsonlite::read_json("https://api.github.com/repos/yutannihilation/winitRPackage/releases/latest")[["tag_name"]]
}

get_download_url <- function() {
  latest_release <- get_latest_release()

  os <- Sys.info()[["sysname"]]
  arch <- Sys.info()[["machine"]]

  binary <- switch(os,
    Windows = "server-Windows-X64.tar.gz",
    Linux   = "server-Linux-X64.tar.gz",
    Darwin  = "server-macOS-ARM64.tar.gz"
  )

  paste(URL_BASE, latest_release, binary, sep = "/")
}

#' Download the server binary
#'
#' @export
download_server <- function() {
  download_tmp_dir <- tempfile()
  extract_tmp_dir <- tempfile()
  on.exit(unlink(download_tmp_dir, recursive = TRUE, force = TRUE), add = TRUE)
  on.exit(unlink(extract_tmp_dir, recursive = TRUE, force = TRUE), add = TRUE)

  # download
  dir.create(download_tmp_dir)
  download_url <- get_download_url()
  archive_file <- file.path(download_tmp_dir, basename(download_url))
  utils::download.file(download_url, destfile = archive_file, mode = "wb")

  # extract and copy
  dst <- server_path()
  dir.create(dirname(dst), showWarnings = FALSE)

  utils::untar(archive_file, exdir = extract_tmp_dir)
  if (Sys.info()[["sysname"]] == "Windows") {
    file.copy(file.path(extract_tmp_dir, "winit_r_package_server.exe"), dst, overwrite = TRUE)
  } else {
    file.copy(file.path(extract_tmp_dir, "winit_r_package_server"), dst, overwrite = TRUE)
  }

  invisible(NULL)
}
