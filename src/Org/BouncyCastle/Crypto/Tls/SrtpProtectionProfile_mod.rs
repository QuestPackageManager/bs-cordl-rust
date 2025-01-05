#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrtpProtectionProfile")]
#[repr(C)]
#[derive(Debug)]
pub struct SrtpProtectionProfile {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrtpProtectionProfile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::SrtpProtectionProfile =>
    "Org.BouncyCastle.Crypto.Tls"."SrtpProtectionProfile"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrtpProtectionProfile")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::SrtpProtectionProfile {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrtpProtectionProfile")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::SrtpProtectionProfile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrtpProtectionProfile")]
impl crate::Org::BouncyCastle::Crypto::Tls::SrtpProtectionProfile {
    pub const SRTP_AEAD_AES_128_GCM: i32 = 7i32;
    pub const SRTP_AEAD_AES_256_GCM: i32 = 8i32;
    pub const SRTP_AES128_CM_HMAC_SHA1_32: i32 = 2i32;
    pub const SRTP_AES128_CM_HMAC_SHA1_80: i32 = 1i32;
    pub const SRTP_NULL_HMAC_SHA1_32: i32 = 6i32;
    pub const SRTP_NULL_HMAC_SHA1_80: i32 = 5i32;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SrtpProtectionProfile")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::SrtpProtectionProfile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
