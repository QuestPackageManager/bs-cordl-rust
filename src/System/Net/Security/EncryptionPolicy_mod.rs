#[cfg(feature = "System+Net+Security+EncryptionPolicy")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionPolicy {
    AllowNoEncryption = 1i32,
    NoEncryption = 2i32,
    RequireEncryption = 0i32,
}
#[cfg(feature = "System+Net+Security+EncryptionPolicy")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Security::EncryptionPolicy =>
    "System.Net.Security"."EncryptionPolicy"
);
