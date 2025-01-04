#[cfg(feature = "Oculus+Platform+AppAgeCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppAgeCategory {
    #[default]
    Ch = 1i32,
    Nch = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+AppAgeCategory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AppAgeCategory =>
    "Oculus.Platform"."AppAgeCategory"
);
