use std::borrow::Cow;

use crate::mime::{normalize_to_lf, AllowedMimeTypes, AsMimeTypes, Error, MimeType};

pub struct Text(pub String);

impl TryFrom<(Vec<u8>, MimeType)> for Text {
    type Error = Error;

    fn try_from((content, mime_type): (Vec<u8>, MimeType)) -> Result<Self, Self::Error> {
        let utf8 = String::from_utf8_lossy(&content);
        let content = match utf8 {
            Cow::Borrowed(_) => String::from_utf8(content).unwrap(),
            Cow::Owned(content) => content,
        };

        // Post-process the content according to mime type.
        let content = match mime_type {
            MimeType::TextPlainUtf8 | MimeType::TextPlain => normalize_to_lf(content),
            MimeType::Utf8String => content,
            MimeType::Other(_) => return Err(Error::Unsupported),
        };
        Ok(Text(content))
    }
}

impl AllowedMimeTypes for Text {
    fn allowed() -> Cow<'static, [MimeType]> {
        Cow::Borrowed(&[MimeType::TextPlainUtf8, MimeType::Utf8String, MimeType::TextPlain])
    }
}

impl AsMimeTypes for Text {
    fn available(&self) -> Cow<'static, [MimeType]> {
        Self::allowed()
    }

    fn as_bytes(&self, mime_type: &MimeType) -> Option<Cow<'static, [u8]>> {
        match mime_type {
            MimeType::TextPlainUtf8 | MimeType::Utf8String | MimeType::TextPlain => {
                Some(Cow::Owned(self.0.as_bytes().to_owned()))
            },
            MimeType::Other(_) => None,
        }
    }
}