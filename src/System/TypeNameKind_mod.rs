#[cfg(feature = "System+TypeNameKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeNameKind {
    FullName = 3i32,
    Name = 0i32,
    SerializationName = 2i32,
    ToString = 1i32,
}
#[cfg(feature = "System+TypeNameKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TypeNameKind => "System"."TypeNameKind"
);
