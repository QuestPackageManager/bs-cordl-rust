#[cfg(feature = "System+Xml+Schema+XsdDateTimeFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XsdDateTimeFlags {
    #[default]
    AllXsd = 255i32,
    Date = 4i32,
    DateTime = 1i32,
    GDay = 64i32,
    GMonth = 128i32,
    GMonthDay = 32i32,
    GYear = 16i32,
    GYearMonth = 8i32,
    Time = 2i32,
    XdrDateTime = 512i32,
    XdrDateTimeNoTz = 256i32,
    XdrTimeNoTz = 1024i32,
}
#[cfg(feature = "System+Xml+Schema+XsdDateTimeFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdDateTimeFlags =>
    "System.Xml.Schema"."XsdDateTimeFlags"
);
