use savvy::savvy;

/// @export
#[savvy]
struct SpawnedWindowController {}

#[savvy]
impl SpawnedWindowController {
    fn new() -> savvy::Result<Self> {
        Err("not implemented".into())
    }

    fn open_window(&mut self, title: &str) -> savvy::Result<()> {
        Err("not implemented".into())
    }

    fn get_window_size(&self) -> savvy::Result<savvy::Sexp> {
        Err("not implemented".into())
    }

    fn close_window(&mut self) -> savvy::Result<()> {
        Err("not implemented".into())
    }
}
