#[cfg(feature = "Mono+Security+X509+X509ChainStatusFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X509ChainStatusFlags {
    InvalidBasicConstraints = 1024i32,
    NoError = 0i32,
    NotSignatureValid = 8i32,
    NotTimeNested = 2i32,
    NotTimeValid = 1i32,
    PartialChain = 65536i32,
    UntrustedRoot = 32i32,
}
#[cfg(feature = "Mono+Security+X509+X509ChainStatusFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::X509ChainStatusFlags =>
    "Mono.Security.X509"."X509ChainStatusFlags"
);
