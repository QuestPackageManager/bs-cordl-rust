#[cfg(feature = "OVR+OpenVR+ETrackedPropertyError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ETrackedPropertyError {
    TrackedProp_BufferTooSmall = 3i32,
    TrackedProp_CannotWriteToWildcards = 12i32,
    TrackedProp_CouldNotContactServer = 6i32,
    TrackedProp_InvalidDevice = 5i32,
    TrackedProp_InvalidOperation = 11i32,
    TrackedProp_NotYetAvailable = 9i32,
    TrackedProp_PermissionDenied = 10i32,
    TrackedProp_StringExceedsMaximumLength = 8i32,
    TrackedProp_Success = 0i32,
    TrackedProp_UnknownProperty = 4i32,
    TrackedProp_ValueNotProvidedByDevice = 7i32,
    TrackedProp_WrongDataType = 1i32,
    TrackedProp_WrongDeviceClass = 2i32,
}
#[cfg(feature = "OVR+OpenVR+ETrackedPropertyError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::ETrackedPropertyError =>
    "OVR.OpenVR"."ETrackedPropertyError"
);
