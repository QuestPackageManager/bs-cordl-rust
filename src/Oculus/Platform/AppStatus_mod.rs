#[cfg(feature = "Oculus+Platform+AppStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppStatus {
    DownloadQueued = 2i32,
    Downloading = 3i32,
    Entitled = 1i32,
    Installed = 5i32,
    Installing = 4i32,
    Uninstalling = 6i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+AppStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AppStatus => "Oculus.Platform"
    ."AppStatus"
);
