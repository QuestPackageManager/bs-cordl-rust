#[cfg(feature = "System+Data+ValueType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Bool = 1i32,
    Date = 7i32,
    Decimal = 5i32,
    Float = 4i32,
    Null = 0i32,
    Numeric = 2i32,
    Object = 6i32,
    Str = 3i32,
    Unknown = -1i32,
}
#[cfg(feature = "System+Data+ValueType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::ValueType => "System.Data"
    ."ValueType"
);
