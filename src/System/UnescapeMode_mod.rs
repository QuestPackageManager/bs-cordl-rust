#[cfg(feature = "System+UnescapeMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnescapeMode {
    CopyOnly = 0i32,
    Escape = 1i32,
    EscapeUnescape = 3i32,
    Unescape = 2i32,
    UnescapeAll = 8i32,
    UnescapeAllOrThrow = 24i32,
    V1ToStringFlag = 4i32,
}
#[cfg(feature = "System+UnescapeMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UnescapeMode => "System"."UnescapeMode"
);
