#[cfg(feature = "Oculus+Platform+MediaContentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaContentType {
    #[default]
    Photo = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+MediaContentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::MediaContentType =>
    "Oculus.Platform"."MediaContentType"
);
