#[cfg(feature = "System+Data+XmlWriteMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlWriteMode {
    #[default]
    DiffGram = 2i32,
    IgnoreSchema = 1i32,
    WriteSchema = 0i32,
}
#[cfg(feature = "System+Data+XmlWriteMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XmlWriteMode => "System.Data"
    ."XmlWriteMode"
);
