#[cfg(feature = "System+Security+Cryptography+CryptoStreamMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CryptoStreamMode {
    #[default]
    Read = 0i32,
    Write = 1i32,
}
#[cfg(feature = "System+Security+Cryptography+CryptoStreamMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::CryptoStreamMode
    => "System.Security.Cryptography"."CryptoStreamMode"
);
