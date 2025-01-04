#[cfg(feature = "Newtonsoft+Json+TypeNameAssemblyFormatHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeNameAssemblyFormatHandling {
    #[default]
    Full = 1i32,
    Simple = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+TypeNameAssemblyFormatHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::TypeNameAssemblyFormatHandling
    => "Newtonsoft.Json"."TypeNameAssemblyFormatHandling"
);
