#[cfg(feature = "System+Security+Cryptography+CspProviderFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CspProviderFlags {
    CreateEphemeralKey = 128i32,
    NoFlags = 0i32,
    NoPrompt = 64i32,
    UseArchivableKey = 16i32,
    UseDefaultKeyContainer = 2i32,
    UseExistingKey = 8i32,
    UseMachineKeyStore = 1i32,
    UseNonExportableKey = 4i32,
    UseUserProtectedKey = 32i32,
}
#[cfg(feature = "System+Security+Cryptography+CspProviderFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::CspProviderFlags
    => "System.Security.Cryptography"."CspProviderFlags"
);
