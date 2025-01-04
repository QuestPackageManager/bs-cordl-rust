#[cfg(feature = "System+Net+Security+SslPolicyErrors")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SslPolicyErrors {
    #[default]
    None = 0i32,
    RemoteCertificateChainErrors = 4i32,
    RemoteCertificateNameMismatch = 2i32,
    RemoteCertificateNotAvailable = 1i32,
}
#[cfg(feature = "System+Net+Security+SslPolicyErrors")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Security::SslPolicyErrors =>
    "System.Net.Security"."SslPolicyErrors"
);
