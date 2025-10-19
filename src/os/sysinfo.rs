use std::sync::Arc;

use tokio::sync::RwLock;

use crate::os::sysinfo;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    os_name: String,
    os_version: String,
    kernel_version: String,
    hostname: String,
    total_memory: usize,
    used_memory: RwLock<usize>,
    update_handle: Option<tokio::task::JoinHandle<()>>,
}

impl SystemInfo {
    /// Create a new SystemInfo instance and start the background update task.
    pub fn new() -> Arc<Self> {
        let sys: SystemInfo = sysinfo::SystemInfo::new();
        sys.refresh_all();
        let res = Arc::new(Self {
            os_name: sys.name().unwrap_or_default(),
            os_version: sys.os_version().unwrap_or_default(),
            kernel_version: sys.kernel_version().unwrap_or_default(),
            hostname: sys.host_name().unwrap_or_default(),
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            update_handle: None,
        });
        let handle = tokio::spawn({
            let sysinfo_clone = Arc::clone(&res);
            async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
                loop {
                    interval.tick().await;
                    sysinfo_clone.update();
                }
            }
        });
        res.update_handle = Some(handle);
        res
    }

    fn update(&mut self) {
        let sys: SystemInfo = sysinfo::SystemInfo::new();
        sys.refresh_all();
        self.used_memory.write().unwrap() = sys.used_memory();
    }

    /// Returns the operating system name.
    pub fn os_name(&self) -> &str {
        &self.os_name
    }

    /// Returns the operating system version.
    pub fn os_version(&self) -> &str {
        &self.os_version
    }

    /// Returns the kernel version.
    pub fn kernel_version(&self) -> &str {
        &self.kernel_version
    }

    /// Returns the hostname.
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    /// Returns the total memory in bytes.
    pub fn total_memory(&self) -> u64 {
        self.total_memory
    }

    /// Returns the used memory in bytes.
    pub async fn used_memory(&self) -> u64 {
        *self.used_memory.read().await
    }
}

impl Drop for SystemInfo {
    fn drop(&mut self) {
        if let Some(handle) = self.update_handle.take() {
            handle.abort();
        }
    }
}
