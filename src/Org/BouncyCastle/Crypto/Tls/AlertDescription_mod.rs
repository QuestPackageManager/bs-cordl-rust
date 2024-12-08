#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AlertDescription")]
#[repr(C)]
#[derive(Debug)]
pub struct AlertDescription {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AlertDescription")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::AlertDescription
    => "Org.BouncyCastle.Crypto.Tls"."AlertDescription"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AlertDescription")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::AlertDescription {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AlertDescription")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::AlertDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AlertDescription")]
impl crate::Org::BouncyCastle::Crypto::Tls::AlertDescription {
    pub const access_denied: u8 = 49u8;
    pub const bad_certificate: u8 = 42u8;
    pub const bad_certificate_hash_value: u8 = 114u8;
    pub const bad_certificate_status_response: u8 = 113u8;
    pub const bad_record_mac: u8 = 20u8;
    pub const certificate_expired: u8 = 45u8;
    pub const certificate_revoked: u8 = 44u8;
    pub const certificate_unknown: u8 = 46u8;
    pub const certificate_unobtainable: u8 = 111u8;
    pub const close_notify: u8 = 0u8;
    pub const decode_error: u8 = 50u8;
    pub const decompression_failure: u8 = 30u8;
    pub const decrypt_error: u8 = 51u8;
    pub const decryption_failed: u8 = 21u8;
    pub const export_restriction: u8 = 60u8;
    pub const handshake_failure: u8 = 40u8;
    pub const illegal_parameter: u8 = 47u8;
    pub const inappropriate_fallback: u8 = 86u8;
    pub const insufficient_security: u8 = 71u8;
    pub const internal_error: u8 = 80u8;
    pub const no_certificate: u8 = 41u8;
    pub const no_renegotiation: u8 = 100u8;
    pub const protocol_version: u8 = 70u8;
    pub const record_overflow: u8 = 22u8;
    pub const unexpected_message: u8 = 10u8;
    pub const unknown_ca: u8 = 48u8;
    pub const unknown_psk_identity: u8 = 115u8;
    pub const unrecognized_name: u8 = 112u8;
    pub const unsupported_certificate: u8 = 43u8;
    pub const unsupported_extension: u8 = 110u8;
    pub const user_canceled: u8 = 90u8;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AlertDescription")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::AlertDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
