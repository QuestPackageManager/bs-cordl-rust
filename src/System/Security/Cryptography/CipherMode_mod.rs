#[cfg(feature = "System+Security+Cryptography+CipherMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CipherMode {
    #[default]
    CBC = 1i32,
    CFB = 4i32,
    CTS = 5i32,
    ECB = 2i32,
    OFB = 3i32,
}
#[cfg(feature = "System+Security+Cryptography+CipherMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::CipherMode =>
    "System.Security.Cryptography"."CipherMode"
);
