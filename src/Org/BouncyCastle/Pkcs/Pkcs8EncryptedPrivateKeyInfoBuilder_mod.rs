#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfoBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs8EncryptedPrivateKeyInfoBuilder {
    __cordl_parent: crate::System::Object,
    pub privateKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfoBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfoBuilder =>
    "Org.BouncyCastle.Pkcs"."Pkcs8EncryptedPrivateKeyInfoBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfoBuilder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfoBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfoBuilder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfoBuilder")]
impl crate::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfoBuilder {
    pub fn Build(
        &mut self,
        encryptor: *mut crate::Org::BouncyCastle::Crypto::ICipherBuilder,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfo = __cordl_object
            .invoke("Build", (encryptor))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        privateKeyInfo: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PrivateKeyInfo1(
        &mut self,
        privateKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyInfo))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray0(
        privateKeyInfo: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKeyInfo))?;
        Ok(__cordl_object)
    }
    pub fn New_PrivateKeyInfo1(
        privateKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKeyInfo))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfoBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfoBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
