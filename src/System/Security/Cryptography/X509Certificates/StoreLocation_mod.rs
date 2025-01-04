#[cfg(feature = "System+Security+Cryptography+X509Certificates+StoreLocation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StoreLocation {
    #[default]
    CurrentUser = 1i32,
    LocalMachine = 2i32,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+StoreLocation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::StoreLocation =>
    "System.Security.Cryptography.X509Certificates"."StoreLocation"
);
