#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct SignerInfoGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub certificate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::X509::X509Certificate,
    >,
    pub contentSigner: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::ISignatureFactory,
    >,
    pub sigId: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
    >,
    pub signedGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    >,
    pub unsignedGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    >,
    pub isDirectSignature: bool,
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::SignerInfoGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "SignerInfoGenerator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::SignerInfoGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New_CmsAttributeTableGenerator_CmsAttributeTableGenerator2(
        sigId: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        >,
        contentSigner: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
        signedGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        >,
        unsignedGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigId, contentSigner, signedGen, unsignedGen))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SignerIdentifier_ISignatureFactory0(
        sigId: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        >,
        signerFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigId, signerFactory))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        sigId: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        >,
        signerFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
        isDirectSignature: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigId, signerFactory, isDirectSignature))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_CmsAttributeTableGenerator_CmsAttributeTableGenerator2(
        &mut self,
        sigId: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        >,
        contentSigner: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
        signedGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        >,
        unsignedGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigId, contentSigner, signedGen, unsignedGen))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SignerIdentifier_ISignatureFactory0(
        &mut self,
        sigId: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        >,
        signerFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigId, signerFactory))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        sigId: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        >,
        signerFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
        isDirectSignature: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigId, signerFactory, isDirectSignature))?;
        Ok(__cordl_ret.into())
    }
    pub fn setAssociatedCertificate(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("setAssociatedCertificate", (certificate))?;
        Ok(__cordl_ret.into())
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
