#[cfg(feature = "System+Net+Http+ClientCertificateOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientCertificateOption {
    Automatic = 1i32,
    Manual = 0i32,
}
#[cfg(feature = "System+Net+Http+ClientCertificateOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::ClientCertificateOption =>
    "System.Net.Http"."ClientCertificateOption"
);
