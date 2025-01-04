#[cfg(feature = "Mono+Security+Interface+TlsProtocols")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TlsProtocols {
    #[default]
    ClientMask = 2688i32,
    ServerMask = 1344i32,
    Tls10 = 192i32,
    Tls10Client = 128i32,
    Tls10Server = 64i32,
    Tls11 = 768i32,
    Tls11Client = 512i32,
    Tls11Server = 256i32,
    Tls12 = 3072i32,
    Tls12Client = 2048i32,
    Tls12Server = 1024i32,
    Zero = 0i32,
}
#[cfg(feature = "Mono+Security+Interface+TlsProtocols")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Interface::TlsProtocols =>
    "Mono.Security.Interface"."TlsProtocols"
);
