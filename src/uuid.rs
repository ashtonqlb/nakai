use std::borrow::Cow;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use rocket::request::FromParam;

/// A unique paste ID.
#[derive(UriDisplayPath)]
pub struct FileID<'a>(Cow<'a, str>);

impl FileID<'_> {
    /// Generate a unique ID as a string representation of a UUID.
    pub fn new() -> FileID<'static> {
        let id = Uuid::new_v4().to_string();
        FileID(Cow::Owned(id))
    }

    /// Returns the path to the paste in `upload/` corresponding to this ID.
    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        Path::new(root).join(self.0.as_ref())
    }
}

/// Returns an instance of `PasteId` if the path segment is a valid UUID.
/// Otherwise returns the invalid ID as the `Err` value.
impl<'a> FromParam<'a> for FileID<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param)
            .map(|_| FileID(param.into()))
            .map_err(|_| param)
    }
}