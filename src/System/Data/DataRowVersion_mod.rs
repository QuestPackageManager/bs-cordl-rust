#[cfg(feature = "System+Data+DataRowVersion")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DataRowVersion {
    #[default]
    Current = 512i32,
    Default = 1536i32,
    Original = 256i32,
    Proposed = 1024i32,
}
#[cfg(feature = "System+Data+DataRowVersion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataRowVersion => "System.Data"
    ."DataRowVersion"
);
