#[cfg(feature = "System+Security+Permissions+PermissionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionState {
    None = 0i32,
    Unrestricted = 1i32,
}
#[cfg(feature = "System+Security+Permissions+PermissionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Permissions::PermissionState
    => "System.Security.Permissions"."PermissionState"
);
