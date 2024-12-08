#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyTransRecipientInformation {
    __cordl_parent: crate::Org::BouncyCastle::Cms::RecipientInformation,
    pub info: *mut crate::Org::BouncyCastle::Asn1::Cms::KeyTransRecipientInfo,
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::KeyTransRecipientInformation => "Org.BouncyCastle.Cms"
    ."KeyTransRecipientInformation"
);
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInformation")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::KeyTransRecipientInformation {
    type Target = crate::Org::BouncyCastle::Cms::RecipientInformation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInformation")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::KeyTransRecipientInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInformation")]
impl crate::Org::BouncyCastle::Cms::KeyTransRecipientInformation {
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
    pub fn GetExchangeEncryptionAlgorithmName(
        &mut self,
        algo: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetExchangeEncryptionAlgorithmName", (algo))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::KeyTransRecipientInfo,
        secureReadable: *mut crate::Org::BouncyCastle::Cms::CmsSecureReadable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, secureReadable))?;
        Ok(__cordl_object)
    }
    pub fn UnwrapKey(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter = __cordl_object
            .invoke("UnwrapKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::KeyTransRecipientInfo,
        secureReadable: *mut crate::Org::BouncyCastle::Cms::CmsSecureReadable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, secureReadable))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::KeyTransRecipientInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
