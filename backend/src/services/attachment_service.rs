use crate::repositories::attachment::AttachmentRepository;
use gritshield::GritComponent;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, GritComponent)]
pub struct AttachmentService {
    pub attachment_repo: Arc<AttachmentRepository>,
}

impl AttachmentService {
    fn storage_dir(&self) -> PathBuf {
        let dir =
            env::var("GRIT_ATTACHMENT_DIR").unwrap_or_else(|_| "data/attachments".to_string());
        let path = PathBuf::from(dir);
        fs::create_dir_all(&path).ok();
        path
    }

    /// Resolve a storage key to an absolute path safely (no traversal).
    pub fn path_for(&self, storage_key: &str) -> Option<PathBuf> {
        let name = PathBuf::from(storage_key);
        if name.components().count() != 1 {
            return None;
        }
        Some(self.storage_dir().join(name))
    }

    pub fn write_bytes(&self, storage_key: &str, bytes: &[u8]) -> io::Result<()> {
        let path = self
            .path_for(storage_key)
            .ok_or_else(|| io::Error::other("invalid storage key"))?;
        fs::write(path, bytes)
    }

    pub fn read_bytes(&self, storage_key: &str) -> io::Result<Vec<u8>> {
        let path = self
            .path_for(storage_key)
            .ok_or_else(|| io::Error::other("invalid storage key"))?;
        fs::read(path)
    }

    pub fn remove_bytes(&self, storage_key: &str) {
        if let Some(path) = self.path_for(storage_key) {
            let _ = fs::remove_file(path);
        }
    }

    /// Generate a unique, single-component storage key for an uploaded file.
    pub fn new_key(&self, filename: &str) -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let ext = PathBuf::from(filename)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e))
            .unwrap_or_default();
        format!("att_{}{}", nanos, ext)
    }
}