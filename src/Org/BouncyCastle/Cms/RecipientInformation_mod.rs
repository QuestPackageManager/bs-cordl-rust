#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct RecipientInformation {
    __cordl_parent: crate::System::Object,
    pub rid: *mut crate::Org::BouncyCastle::Cms::RecipientID,
    pub keyEncAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub secureReadable: *mut crate::Org::BouncyCastle::Cms::CmsSecureReadable,
    pub resultMac: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::RecipientInformation =>
    "Org.BouncyCastle.Cms"."RecipientInformation"
);
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformation")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::RecipientInformation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformation")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::RecipientInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformation")]
impl crate::Org::BouncyCastle::Cms::RecipientInformation {
    pub fn GetContent(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetContent", (key))?;
        Ok(__cordl_ret)
    }
    pub fn GetContentAlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetContentAlgorithmName", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetContentFromSessionKey(
        &mut self,
        sKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsTypedStream,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsTypedStream = __cordl_object
            .invoke("GetContentFromSessionKey", (sKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetContentStream(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsTypedStream,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsTypedStream = __cordl_object
            .invoke("GetContentStream", (key))?;
        Ok(__cordl_ret)
    }
    pub fn GetMac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetMac", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        keyEncAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        secureReadable: *mut crate::Org::BouncyCastle::Cms::CmsSecureReadable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyEncAlg, secureReadable))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        keyEncAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        secureReadable: *mut crate::Org::BouncyCastle::Cms::CmsSecureReadable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyEncAlg, secureReadable))?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyEncryptionAlgOid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_KeyEncryptionAlgOid", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyEncryptionAlgParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("get_KeyEncryptionAlgParams", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyEncryptionAlgorithmID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_KeyEncryptionAlgorithmID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RecipientID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Cms::RecipientID> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::RecipientID = __cordl_object
            .invoke("get_RecipientID", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::RecipientInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}