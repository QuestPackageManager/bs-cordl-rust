#[cfg(feature = "Newtonsoft+Json+ReadType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReadType {
    #[default]
    Read = 0i32,
    ReadAsBoolean = 9i32,
    ReadAsBytes = 3i32,
    ReadAsDateTime = 6i32,
    ReadAsDateTimeOffset = 7i32,
    ReadAsDecimal = 5i32,
    ReadAsDouble = 8i32,
    ReadAsInt32 = 1i32,
    ReadAsInt64 = 2i32,
    ReadAsString = 4i32,
}
#[cfg(feature = "Newtonsoft+Json+ReadType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::ReadType => "Newtonsoft.Json"
    ."ReadType"
);
