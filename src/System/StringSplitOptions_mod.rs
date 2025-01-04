#[cfg(feature = "System+StringSplitOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StringSplitOptions {
    #[default]
    None = 0i32,
    RemoveEmptyEntries = 1i32,
}
#[cfg(feature = "System+StringSplitOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::StringSplitOptions => "System"
    ."StringSplitOptions"
);
