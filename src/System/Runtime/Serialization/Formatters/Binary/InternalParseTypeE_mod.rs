#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalParseTypeE")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InternalParseTypeE {
    #[default]
    Body = 11i32,
    BodyEnd = 12i32,
    Empty = 0i32,
    Envelope = 9i32,
    EnvelopeEnd = 10i32,
    Headers = 6i32,
    HeadersEnd = 7i32,
    Member = 3i32,
    MemberEnd = 5i32,
    Object = 2i32,
    ObjectEnd = 4i32,
    SerializedStreamHeader = 1i32,
    SerializedStreamHeaderEnd = 8i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalParseTypeE")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::InternalParseTypeE =>
    "System.Runtime.Serialization.Formatters.Binary"."InternalParseTypeE"
);
