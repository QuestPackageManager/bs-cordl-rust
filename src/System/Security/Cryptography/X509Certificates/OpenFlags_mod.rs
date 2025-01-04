#[cfg(feature = "System+Security+Cryptography+X509Certificates+OpenFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OpenFlags {
    #[default]
    IncludeArchived = 8i32,
    MaxAllowed = 2i32,
    OpenExistingOnly = 4i32,
    ReadOnly = 0i32,
    ReadWrite = 1i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+OpenFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::OpenFlags =>
    "System.Security.Cryptography.X509Certificates"."OpenFlags"
);
