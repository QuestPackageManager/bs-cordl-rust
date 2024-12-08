#[cfg(feature = "System+UriKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UriKind {
    Absolute = 1i32,
    Relative = 2i32,
    RelativeOrAbsolute = 0i32,
}
#[cfg(feature = "System+UriKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UriKind => "System"."UriKind"
);
