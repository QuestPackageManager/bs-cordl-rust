#[cfg(feature = "System+StringComparison")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StringComparison {
    #[default]
    CurrentCulture = 0i32,
    CurrentCultureIgnoreCase = 1i32,
    InvariantCulture = 2i32,
    InvariantCultureIgnoreCase = 3i32,
    Ordinal = 4i32,
    OrdinalIgnoreCase = 5i32,
}
#[cfg(feature = "System+StringComparison")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::StringComparison => "System"
    ."StringComparison"
);
