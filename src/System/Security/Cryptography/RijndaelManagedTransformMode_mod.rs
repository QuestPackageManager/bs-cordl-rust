#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransformMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RijndaelManagedTransformMode {
    #[default]
    Decrypt = 1i32,
    Encrypt = 0i32,
}
#[cfg(feature = "System+Security+Cryptography+RijndaelManagedTransformMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::RijndaelManagedTransformMode =>
    "System.Security.Cryptography"."RijndaelManagedTransformMode"
);
