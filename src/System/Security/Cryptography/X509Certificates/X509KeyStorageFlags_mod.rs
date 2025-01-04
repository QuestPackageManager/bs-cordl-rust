#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyStorageFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X509KeyStorageFlags {
    #[default]
    DefaultKeySet = 0i32,
    EphemeralKeySet = 32i32,
    Exportable = 4i32,
    MachineKeySet = 2i32,
    PersistKeySet = 16i32,
    UserKeySet = 1i32,
    UserProtected = 8i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyStorageFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509KeyStorageFlags =>
    "System.Security.Cryptography.X509Certificates"."X509KeyStorageFlags"
);
