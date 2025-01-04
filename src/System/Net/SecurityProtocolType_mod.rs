#[cfg(feature = "System+Net+SecurityProtocolType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SecurityProtocolType {
    #[default]
    Ssl3 = 48i32,
    SystemDefault = 0i32,
    Tls = 192i32,
    Tls11 = 768i32,
    Tls12 = 3072i32,
    Tls13 = 12288i32,
}
#[cfg(feature = "System+Net+SecurityProtocolType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::SecurityProtocolType =>
    "System.Net"."SecurityProtocolType"
);
