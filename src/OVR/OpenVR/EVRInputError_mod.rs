#[cfg(feature = "OVR+OpenVR+EVRInputError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRInputError {
    BufferTooSmall = 14i32,
    IPCError = 7i32,
    InvalidBoneCount = 11i32,
    InvalidCompressedData = 12i32,
    InvalidDevice = 9i32,
    InvalidHandle = 3i32,
    InvalidParam = 4i32,
    InvalidSkeleton = 10i32,
    MaxCapacityReached = 6i32,
    MismatchedActionManifest = 15i32,
    MissingSkeletonData = 16i32,
    NameNotFound = 1i32,
    NoActiveActionSet = 8i32,
    NoData = 13i32,
    NoSteam = 5i32,
    None = 0i32,
    WrongType = 2i32,
}
#[cfg(feature = "OVR+OpenVR+EVRInputError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRInputError => "OVR.OpenVR"
    ."EVRInputError"
);
