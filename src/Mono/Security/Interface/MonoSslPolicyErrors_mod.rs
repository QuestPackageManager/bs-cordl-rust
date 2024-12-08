#[cfg(feature = "Mono+Security+Interface+MonoSslPolicyErrors")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonoSslPolicyErrors {
    None = 0i32,
    RemoteCertificateChainErrors = 4i32,
    RemoteCertificateNameMismatch = 2i32,
    RemoteCertificateNotAvailable = 1i32,
}
#[cfg(feature = "Mono+Security+Interface+MonoSslPolicyErrors")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Interface::MonoSslPolicyErrors
    => "Mono.Security.Interface"."MonoSslPolicyErrors"
);
