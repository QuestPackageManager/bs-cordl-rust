#[cfg(feature = "System+Security+Cryptography+PaddingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaddingMode {
    ANSIX923 = 4i32,
    ISO10126 = 5i32,
    None = 1i32,
    PKCS7 = 2i32,
    Zeros = 3i32,
}
#[cfg(feature = "System+Security+Cryptography+PaddingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::PaddingMode =>
    "System.Security.Cryptography"."PaddingMode"
);
