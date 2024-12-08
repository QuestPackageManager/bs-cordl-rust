#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedNameFlags"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X500DistinguishedNameFlags {
    DoNotUsePlusSign = 32i32,
    DoNotUseQuotes = 64i32,
    ForceUTF8Encoding = 16384i32,
    None = 0i32,
    Reversed = 1i32,
    UseCommas = 128i32,
    UseNewLines = 256i32,
    UseSemicolons = 16i32,
    UseT61Encoding = 8192i32,
    UseUTF8Encoding = 4096i32,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedNameFlags"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X500DistinguishedNameFlags =>
    "System.Security.Cryptography.X509Certificates"."X500DistinguishedNameFlags"
);
