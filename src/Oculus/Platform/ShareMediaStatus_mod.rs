#[cfg(feature = "Oculus+Platform+ShareMediaStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShareMediaStatus {
    #[default]
    Canceled = 2i32,
    Shared = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+ShareMediaStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::ShareMediaStatus =>
    "Oculus.Platform"."ShareMediaStatus"
);
