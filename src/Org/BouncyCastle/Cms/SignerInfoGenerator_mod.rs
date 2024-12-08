#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct SignerInfoGenerator {
    __cordl_parent: crate::System::Object,
    pub certificate: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    pub contentSigner: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
    pub sigId: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
    pub signedGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    pub unsignedGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    pub isDirectSignature: bool,
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::SignerInfoGenerator =>
    "Org.BouncyCastle.Cms"."SignerInfoGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::SignerInfoGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::SignerInfoGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGenerator")]
impl crate::Org::BouncyCastle::Cms::SignerInfoGenerator {
    pub fn setAssociatedCertificate(
        &mut self,
        certificate: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("setAssociatedCertificate", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SignerIdentifier_ISignatureFactory0(
        &mut self,
        sigId: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        signerFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigId, signerFactory))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        sigId: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        signerFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        isDirectSignature: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigId, signerFactory, isDirectSignature))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CmsAttributeTableGenerator_CmsAttributeTableGenerator2(
        &mut self,
        sigId: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        contentSigner: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        signedGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigId, contentSigner, signedGen, unsignedGen))?;
        Ok(__cordl_ret)
    }
    pub fn New_SignerIdentifier_ISignatureFactory0(
        sigId: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        signerFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigId, signerFactory))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        sigId: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        signerFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        isDirectSignature: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigId, signerFactory, isDirectSignature))?;
        Ok(__cordl_object)
    }
    pub fn New_CmsAttributeTableGenerator_CmsAttributeTableGenerator2(
        sigId: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        contentSigner: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        signedGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigId, contentSigner, signedGen, unsignedGen))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::SignerInfoGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
