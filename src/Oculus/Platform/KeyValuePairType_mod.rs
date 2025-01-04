#[cfg(feature = "Oculus+Platform+KeyValuePairType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyValuePairType {
    #[default]
    Double = 2i32,
    Int = 1i32,
    String = 0i32,
    Unknown = 3i32,
}
#[cfg(feature = "Oculus+Platform+KeyValuePairType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::KeyValuePairType =>
    "Oculus.Platform"."KeyValuePairType"
);
