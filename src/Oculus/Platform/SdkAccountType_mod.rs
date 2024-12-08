#[cfg(feature = "Oculus+Platform+SdkAccountType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkAccountType {
    FacebookGameroom = 2i32,
    Oculus = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+SdkAccountType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::SdkAccountType =>
    "Oculus.Platform"."SdkAccountType"
);
