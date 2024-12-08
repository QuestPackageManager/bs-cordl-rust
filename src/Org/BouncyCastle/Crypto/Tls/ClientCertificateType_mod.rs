#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ClientCertificateType")]
#[repr(C)]
#[derive(Debug)]
pub struct ClientCertificateType {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ClientCertificateType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::ClientCertificateType =>
    "Org.BouncyCastle.Crypto.Tls"."ClientCertificateType"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ClientCertificateType")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::ClientCertificateType {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ClientCertificateType")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::ClientCertificateType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ClientCertificateType")]
impl crate::Org::BouncyCastle::Crypto::Tls::ClientCertificateType {
    pub const dss_ephemeral_dh_RESERVED: u8 = 6u8;
    pub const dss_fixed_dh: u8 = 4u8;
    pub const dss_sign: u8 = 2u8;
    pub const ecdsa_fixed_ecdh: u8 = 66u8;
    pub const ecdsa_sign: u8 = 64u8;
    pub const fortezza_dms_RESERVED: u8 = 20u8;
    pub const rsa_ephemeral_dh_RESERVED: u8 = 5u8;
    pub const rsa_fixed_dh: u8 = 3u8;
    pub const rsa_fixed_ecdh: u8 = 65u8;
    pub const rsa_sign: u8 = 1u8;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ClientCertificateType")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::ClientCertificateType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
