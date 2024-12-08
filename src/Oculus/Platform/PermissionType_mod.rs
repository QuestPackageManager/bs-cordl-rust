#[cfg(feature = "Oculus+Platform+PermissionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionType {
    Microphone = 1i32,
    Unknown = 0i32,
    WriteExternalStorage = 2i32,
}
#[cfg(feature = "Oculus+Platform+PermissionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::PermissionType =>
    "Oculus.Platform"."PermissionType"
);
