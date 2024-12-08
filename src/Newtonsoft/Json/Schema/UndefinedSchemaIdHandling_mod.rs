#[cfg(feature = "Newtonsoft+Json+Schema+UndefinedSchemaIdHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UndefinedSchemaIdHandling {
    None = 0i32,
    UseAssemblyQualifiedName = 2i32,
    UseTypeName = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+Schema+UndefinedSchemaIdHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Schema::UndefinedSchemaIdHandling => "Newtonsoft.Json.Schema"
    ."UndefinedSchemaIdHandling"
);
