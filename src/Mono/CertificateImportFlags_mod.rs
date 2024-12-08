#[cfg(feature = "Mono+CertificateImportFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateImportFlags {
    DisableAutomaticFallback = 2i32,
    DisableNativeBackend = 1i32,
    None = 0i32,
}
#[cfg(feature = "Mono+CertificateImportFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::CertificateImportFlags => "Mono"
    ."CertificateImportFlags"
);
