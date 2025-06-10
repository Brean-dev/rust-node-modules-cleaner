use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Gets the size of a file in bytes
///
/// # Arguments
/// * `path` - Path to the file
///
/// # Returns
/// * `Ok(u64)` - Size of the file in bytes
/// * `Err` - If the path doesn't exist, isn't accessible, or is not a file
///
/// # Notes
/// This function verifies that the path is actually a file, not a directory
/// or other file system object, before returning its size.
pub fn get_file_size_bytes<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    // Get file metadata (will fail if file doesn't exist or isn't accessible)
    let metadata = fs::metadata(path)?;

    // Check if it's a file (not a directory, symlink, etc.)
    if metadata.is_file() {
        Ok(metadata.len())
    } else {
        // Return a descriptive error if the path exists but isn't a file
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Not a file"))
    }
}

/// Recursively calculates the total size of a directory and all its contents
///
/// # Arguments
/// * `path` - Path to the directory
///
/// # Returns
/// * `Ok(u64)` - Total size of all files in the directory (and subdirectories) in bytes
/// * `Err` - If the path doesn't exist, isn't accessible, or can't be read
///
/// # Performance Considerations
/// - This function recursively traverses the entire directory tree, which can be
///   slow and memory-intensive for large directory structures
/// - No protection against cycles in the file system (e.g., symbolic links)
/// - For very large directories, consider using a non-recursive or streaming approach
pub fn get_directory_size_bytes<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let mut total_size = 0;

    // Iterate through all entries in the directory
    for entry in fs::read_dir(path)? {
        // Unwrap the entry (propagates any errors)
        let entry = entry?;
        let path = entry.path();

        // For files, add their size to the total
        if path.is_file() {
            total_size += get_file_size_bytes(&path)?;
        }
        // For directories, recursively calculate their size and add to the total
        else if path.is_dir() {
            total_size += get_directory_size_bytes(&path)?;
        }
        // Other file types (symlinks, pipes, etc.) are ignored
    }

    Ok(total_size)
}

/// Converts bytes to megabytes
///
/// # Arguments
/// * `bytes` - Size in bytes
///
/// # Returns
/// * `f64` - Size in megabytes (using 1 MB = 1,048,576 bytes)
pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

/// Analyzes a path and returns its size information
///
/// # Arguments
/// * `path` - Path to analyze (file or directory)
///
/// # Returns
/// * `Ok((u64, f64))` - Tuple containing size in bytes and in megabytes
/// * `Err` - If the path doesn't exist, isn't accessible, or has other issues
/// *  Gets the size of a single path (file or directory)
pub fn get_path_size<P: AsRef<Path>>(path: P) -> io::Result<(u64, f64)> {
    let path_ref = path.as_ref();

    let size_bytes = if path_ref.is_file() {
        get_file_size_bytes(path_ref)?
    } else if path_ref.is_dir() {
        get_directory_size_bytes(path_ref)?
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path does not exist or is not a file or directory",
        ));
    };

    let size_mb = bytes_to_mb(size_bytes);
    Ok((size_bytes, size_mb))
}

/// Gets the total size of multiple paths (files and/or directories)
///
/// # Arguments
/// * `paths` - A vector of paths to analyze
///
/// # Returns
/// * `Ok((u64, f64))` - Tuple containing total size in bytes and in megabytes
/// * `Err` - If any path is invalid or inaccessible
pub fn get_paths_size(paths: &[PathBuf]) -> io::Result<(u64, f64)> {
    let mut total_bytes = 0;

    for path in paths {
        let (bytes, _) = get_path_size(path)?;
        total_bytes += bytes;
    }

    let total_mb = bytes_to_mb(total_bytes);
    Ok((total_bytes, total_mb))
}
fn main() {}
