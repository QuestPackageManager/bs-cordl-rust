#[cfg(feature = "System+Data+SqlTypes+SqlBytesCharsState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SqlBytesCharsState {
    #[default]
    Buffer = 1i32,
    Null = 0i32,
    Stream = 3i32,
}
#[cfg(feature = "System+Data+SqlTypes+SqlBytesCharsState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SqlTypes::SqlBytesCharsState =>
    "System.Data.SqlTypes"."SqlBytesCharsState"
);
