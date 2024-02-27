use std::path::PathBuf;
use serde::Deserialize;
use serde_json::Error;

/// Deserializes JSON content from a file specified by the given `target` path.
///
/// # Arguments
/// * `target` - A reference to a `PathBuf` object representing the path to the JSON file.
///
/// # Generic Parameters
/// * `C` - The type to deserialize the JSON content to. It must implement the `Deserialize` trait.
///
/// # Returns
/// `Result` containing the deserialized object of type `C` on success, or an `Error` if deserialization fails.
///
/// # Panics
/// If `std::fs::read_to_string` fails to read the file, function will panic
pub fn json<C>(target: &PathBuf) -> Result<C, Error>
    where for<'a> C: Deserialize<'a>
{
    let content: String = std::fs::read_to_string(target).unwrap_or_else(
        |_| panic!("Failed to read {}", &target.display())
    );

    serde_json::from_str(&content)
}
