#[cfg(feature = "Oculus+Platform+PermissionGrantStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionGrantStatus {
    Blocked = 3i32,
    Denied = 2i32,
    Granted = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+PermissionGrantStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::PermissionGrantStatus =>
    "Oculus.Platform"."PermissionGrantStatus"
);
