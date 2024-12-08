#[cfg(feature = "Oculus+Platform+AccountAgeCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountAgeCategory {
    Ad = 3i32,
    Ch = 1i32,
    Tn = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+AccountAgeCategory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AccountAgeCategory =>
    "Oculus.Platform"."AccountAgeCategory"
);
