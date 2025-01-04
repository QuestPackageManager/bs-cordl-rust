#[cfg(feature = "System+Runtime+Serialization+Formatters+FormatterAssemblyStyle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormatterAssemblyStyle {
    #[default]
    Full = 1i32,
    Simple = 0i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+FormatterAssemblyStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::FormatterAssemblyStyle =>
    "System.Runtime.Serialization.Formatters"."FormatterAssemblyStyle"
);
