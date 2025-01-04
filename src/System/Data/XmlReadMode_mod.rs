#[cfg(feature = "System+Data+XmlReadMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlReadMode {
    #[default]
    Auto = 0i32,
    DiffGram = 4i32,
    Fragment = 5i32,
    IgnoreSchema = 2i32,
    InferSchema = 3i32,
    InferTypedSchema = 6i32,
    ReadSchema = 1i32,
}
#[cfg(feature = "System+Data+XmlReadMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XmlReadMode => "System.Data"
    ."XmlReadMode"
);
