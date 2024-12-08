#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509NameType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X509NameType {
    DnsFromAlternativeName = 4i32,
    DnsName = 3i32,
    EmailName = 1i32,
    SimpleName = 0i32,
    UpnName = 2i32,
    UrlName = 5i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509NameType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509NameType =>
    "System.Security.Cryptography.X509Certificates"."X509NameType"
);
