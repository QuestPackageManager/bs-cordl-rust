#[cfg(feature = "System+Data+SchemaFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaFormat {
    Public = 1i32,
    Remoting = 2i32,
    RemotingSkipSchema = 4i32,
    WebService = 3i32,
    WebServiceSkipSchema = 5i32,
}
#[cfg(feature = "System+Data+SchemaFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SchemaFormat => "System.Data"
    ."SchemaFormat"
);
