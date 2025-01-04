#[cfg(feature = "System+Runtime+Serialization+Formatters+FormatterTypeStyle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormatterTypeStyle {
    #[default]
    TypesAlways = 1i32,
    TypesWhenNeeded = 0i32,
    XsdString = 2i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+FormatterTypeStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::FormatterTypeStyle =>
    "System.Runtime.Serialization.Formatters"."FormatterTypeStyle"
);
