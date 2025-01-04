#[cfg(feature = "UserAgeCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserAgeCategory {
    #[default]
    Adult = 3i32,
    Child = 1i32,
    Teen = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "UserAgeCategory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UserAgeCategory => ""
    ."UserAgeCategory"
);
