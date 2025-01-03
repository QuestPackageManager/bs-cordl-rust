#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct PasswordRecipientInformation {
    __cordl_parent: crate::Org::BouncyCastle::Cms::RecipientInformation,
    pub info: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::PasswordRecipientInfo,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::PasswordRecipientInformation => "Org.BouncyCastle.Cms"
    ."PasswordRecipientInformation"
);
#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInformation")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::PasswordRecipientInformation {
    type Target = crate::Org::BouncyCastle::Cms::RecipientInformation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInformation")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::PasswordRecipientInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInformation")]
impl crate::Org::BouncyCastle::Cms::PasswordRecipientInformation {
    pub fn GetContentStream(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsTypedStream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsTypedStream,
        > = __cordl_object.invoke("GetContentStream", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        info: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::PasswordRecipientInfo,
        >,
        secureReadable: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSecureReadable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, secureReadable))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::PasswordRecipientInfo,
        >,
        secureReadable: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSecureReadable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, secureReadable))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyDerivationAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = __cordl_object.invoke("get_KeyDerivationAlgorithm", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::PasswordRecipientInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
