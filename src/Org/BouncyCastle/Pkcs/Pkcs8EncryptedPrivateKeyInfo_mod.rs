#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs8EncryptedPrivateKeyInfo {
    __cordl_parent: crate::System::Object,
    pub encryptedPrivateKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfo => "Org.BouncyCastle.Pkcs"
    ."Pkcs8EncryptedPrivateKeyInfo"
);
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfo")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfo")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfo")]
impl crate::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfo {
    pub fn GetEncoded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetEncoded", ())?;
        Ok(__cordl_ret)
    }
    pub fn DecryptPrivateKeyInfo(
        &mut self,
        inputDecryptorProvider: *mut crate::Org::BouncyCastle::Crypto::IDecryptorBuilderProvider,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo = __cordl_object
            .invoke("DecryptPrivateKeyInfo", (inputDecryptorProvider))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_EncryptedPrivateKeyInfo0(
        &mut self,
        encryptedPrivateKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encryptedPrivateKeyInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        encryptedPrivateKeyInfo: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encryptedPrivateKeyInfo))?;
        Ok(__cordl_ret)
    }
    pub fn ToAsn1Structure(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo = __cordl_object
            .invoke("ToAsn1Structure", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEncryptedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetEncryptedData", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_EncryptedPrivateKeyInfo0(
        encryptedPrivateKeyInfo: *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encryptedPrivateKeyInfo))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray1(
        encryptedPrivateKeyInfo: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encryptedPrivateKeyInfo))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs8EncryptedPrivateKeyInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::Pkcs8EncryptedPrivateKeyInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
