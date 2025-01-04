#[cfg(feature = "System+Security+SecurityElementType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SecurityElementType {
    #[default]
    Comment = 2i32,
    Format = 1i32,
    Regular = 0i32,
}
#[cfg(feature = "System+Security+SecurityElementType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::SecurityElementType =>
    "System.Security"."SecurityElementType"
);
