use std::fs;
use std::path::PathBuf;

use ratatui::text::Span;

use super::App;

#[derive(Clone)]
pub struct Meta {
    last_modified: Option<std::time::SystemTime>,
    file_changed: bool,
    pub path: std::path::PathBuf,
}

impl Meta {
    pub fn has_changed(&mut self) -> Result<bool, std::io::Error> {
        let modified = fs::metadata(&self.path)?.modified();

        match (modified.ok(), self.last_modified) {
            (Some(modified), Some(last_modified)) => {
                if modified > last_modified {
                    self.file_changed = true;
                }

                self.last_modified = Some(modified);
            }
            (Some(modified), None) => {
                self.file_changed = true;
                self.last_modified = Some(modified);
            }
            _ => (),
        }

        Ok(self.file_changed)
    }

    pub fn reset(&mut self) {
        self.file_changed = false;
    }
}

impl TryFrom<PathBuf> for Meta {
    type Error = std::io::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let meta = Self {
            last_modified: fs::metadata(&value)?.modified().ok(),
            file_changed: false,
            path: value,
        };

        Ok(meta)
    }
}

impl From<&App> for Meta {
    fn from(app: &App) -> Self {
        app.file.clone()
    }
}

impl From<Meta> for Vec<Span<'_>> {
    fn from(meta: Meta) -> Self {
        let mut metas = vec![Span::from(format!(" {} ", meta.path.to_string_lossy()))];

        if meta.file_changed {
            metas.push(Span::from(" "))
        }

        metas
    }
}
