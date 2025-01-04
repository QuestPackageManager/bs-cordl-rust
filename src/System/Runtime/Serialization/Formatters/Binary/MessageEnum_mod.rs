#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MessageEnum")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageEnum {
    #[default]
    ArgsInArray = 8i32,
    ArgsInline = 2i32,
    ArgsIsArray = 4i32,
    ContextInArray = 64i32,
    ContextInline = 32i32,
    ExceptionInArray = 8192i32,
    GenericMethod = 32768i32,
    MethodSignatureInArray = 128i32,
    NoArgs = 1i32,
    NoContext = 16i32,
    NoReturnValue = 512i32,
    PropertyInArray = 256i32,
    ReturnValueInArray = 4096i32,
    ReturnValueInline = 2048i32,
    ReturnValueVoid = 1024i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MessageEnum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::MessageEnum =>
    "System.Runtime.Serialization.Formatters.Binary"."MessageEnum"
);
