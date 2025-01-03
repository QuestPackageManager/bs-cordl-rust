#[cfg(feature = "Org+BouncyCastle+Operators+CmsKeyTransRecipientInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsKeyTransRecipientInfoGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Cms::KeyTransRecipientInfoGenerator,
    pub keyWrapper: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IKeyWrapper,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsKeyTransRecipientInfoGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Operators::CmsKeyTransRecipientInfoGenerator =>
    "Org.BouncyCastle.Operators"."CmsKeyTransRecipientInfoGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Operators+CmsKeyTransRecipientInfoGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Operators::CmsKeyTransRecipientInfoGenerator {
    type Target = crate::Org::BouncyCastle::Cms::KeyTransRecipientInfoGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsKeyTransRecipientInfoGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Operators::CmsKeyTransRecipientInfoGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsKeyTransRecipientInfoGenerator")]
impl crate::Org::BouncyCastle::Operators::CmsKeyTransRecipientInfoGenerator {
    pub fn GenerateWrappedKey(
        &mut self,
        contentKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GenerateWrappedKey", (contentKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray1(
        subjectKeyID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        keyWrapper: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IKeyWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subjectKeyID, keyWrapper))?;
        Ok(__cordl_object.into())
    }
    pub fn New_X509Certificate0(
        recipCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        keyWrapper: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IKeyWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (recipCert, keyWrapper))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        subjectKeyID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        keyWrapper: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IKeyWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subjectKeyID, keyWrapper))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_X509Certificate0(
        &mut self,
        recipCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        keyWrapper: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IKeyWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (recipCert, keyWrapper))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AlgorithmDetails(
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
        > = __cordl_object.invoke("get_AlgorithmDetails", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsKeyTransRecipientInfoGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Operators::CmsKeyTransRecipientInfoGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
