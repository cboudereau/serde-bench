//https://github.com/serde-rs/json/issues/404#issuecomment-674293399
mod types;
pub use types::Json;

mod serde_json_stream_deserializer;
pub use serde_json_stream_deserializer::JsonIterator;

mod serde_json_simple_deserializer;
pub use serde_json_simple_deserializer::{read_from_file, readone_from_file};

mod serde_json_stream_deserializer2;

pub use serde_json_stream_deserializer2::iter_json_array;