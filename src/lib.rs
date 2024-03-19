mod constants;
mod encoding;
mod file;
mod text_data;
mod utf16;

pub use encoding::Encoding;
pub use file::read_to_string;
pub use file::read_to_text_data;
pub use file::File;
pub use file::FileContent;
pub use file::FileError;
pub use text_data::TextData;
pub use text_data::TextDataError;
