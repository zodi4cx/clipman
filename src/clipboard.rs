//! Utils for intereacting with the clipboard in X11 systems. As the X11 protocol
//! is supported in Wayland, this functionality should work there too, although it
//! is currently untested.

use arboard::{Error, SetExtLinux};
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, fs::File, io, path::Path};

/// Holds the bytes and basic metadata of an image
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl<'a> From<arboard::ImageData<'a>> for ImageData {
    fn from(value: arboard::ImageData<'a>) -> Self {
        ImageData {
            width: value.width,
            height: value.height,
            data: value.bytes.into_owned(),
        }
    }
}

/// Possible values that the clipboard content may hold
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClipContent {
    Text(String),
    Image(ImageData),
}

impl From<String> for ClipContent {
    fn from(value: String) -> Self {
        ClipContent::Text(value)
    }
}

impl From<ImageData> for ClipContent {
    fn from(value: ImageData) -> Self {
        ClipContent::Image(value)
    }
}

/// Represents a clipboard with multiple states that can be retrieved or modified
/// at any time.
///
/// # Example
///
/// ```rust
/// use clipman::clipboard::{Clipboard, ClipContent};
/// 
/// let mut clip = Clipboard::new();
/// let test_data = "This is a test!".to_owned();
/// clip.insert(1, test_data.clone());
/// 
/// let content = clip.get(1).unwrap();
/// assert_eq!(&ClipContent::Text(test_data), content);
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Clipboard(HashMap<u32, ClipContent>);

impl Clipboard {
    /// Creates a new Clipboard with a blank state.
    pub fn new() -> Clipboard {
        Default::default()
    }

    /// Saves the given content to the clipboard map.
    pub fn insert<T>(&mut self, index: u32, data: T)
    where
        T: Into<ClipContent>,
    {
        self.0.insert(index, data.into());
    }

    /// Retrieves content from the clipboard map. If the index is not
    /// found, None is returned instead.
    pub fn get(&self, index: u32) -> Option<&ClipContent> {
        self.0.get(&index)
    }

    /// Creates a new Clipboard instance from a serialized file.
    pub fn open(filepath: &Path) -> Result<Clipboard, io::Error> {
        let file = File::open(filepath)?;
        rmp_serde::from_read(&file).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "failed to deserialize Clipboard",
            )
        })
    }

    /// Stores the Clipboard instance as a serialized file.
    pub fn save(&self, filepath: &Path) -> Result<(), io::Error> {
        let mut file = File::create(filepath)?;
        self.serialize(&mut Serializer::new(&mut file))
            .map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "failed to serialize Clipboard")
            })
    }
}

/// Retrieves the current content stored in the system clipboard.
///
/// # Panics
///
/// This function may panic if access to the clipboard cannot be obtained.
pub fn get_clipboard() -> Result<ClipContent, Error> {
    let mut clipboard = arboard::Clipboard::new().expect("Can't access the clipboard");
    let data = match clipboard.get_text() {
        Ok(text) => ClipContent::Text(text),
        Err(Error::ContentNotAvailable) => ClipContent::Image(clipboard.get_image()?.into()),
        Err(err) => return Err(err),
    };
    Ok(data)
}

/// Sets the selected content into the system clipboard.
///
/// # Panics
///
/// This function may panic if access to the clipboard cannot be obtained.
pub fn set_clipboard(data: &ClipContent) -> Result<(), Error>
{
    let mut clipboard = arboard::Clipboard::new().expect("Can't access the clipboard");
    if cfg!(target_os = "linux") {
        match data {
            ClipContent::Text(data) => clipboard.set().wait().text(data),
            ClipContent::Image(data) => clipboard.set().wait().image(arboard::ImageData {
                width: data.width,
                height: data.height,
                bytes: Cow::from(data.data.as_slice()),
            }),
        }
    } else {
        match data {
            ClipContent::Text(data) => clipboard.set_text(data),
            ClipContent::Image(data) => clipboard.set_image(arboard::ImageData {
                width: data.width,
                height: data.height,
                bytes: Cow::from(data.data.as_slice()),
            }),
        }
    }
}
