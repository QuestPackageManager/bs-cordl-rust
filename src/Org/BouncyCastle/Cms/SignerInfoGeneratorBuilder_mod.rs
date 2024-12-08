#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGeneratorBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct SignerInfoGeneratorBuilder {
    __cordl_parent: crate::System::Object,
    pub directSignature: bool,
    pub signedGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    pub unsignedGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGeneratorBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder => "Org.BouncyCastle.Cms"
    ."SignerInfoGeneratorBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGeneratorBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGeneratorBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGeneratorBuilder")]
impl crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder {
    pub fn Build_X509Certificate0(
        &mut self,
        contentSigner: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        certificate: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::SignerInfoGenerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerInfoGenerator = __cordl_object
            .invoke("Build", (contentSigner, certificate))?;
        Ok(__cordl_ret)
    }
    pub fn Build_Il2CppArray1(
        &mut self,
        signerFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        subjectKeyIdentifier: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::SignerInfoGenerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerInfoGenerator = __cordl_object
            .invoke("Build", (signerFactory, subjectKeyIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn CreateGenerator(
        &mut self,
        contentSigner: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        sigId: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::SignerInfoGenerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerInfoGenerator = __cordl_object
            .invoke("CreateGenerator", (contentSigner, sigId))?;
        Ok(__cordl_ret)
    }
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
    pub fn SetDirectSignature(
        &mut self,
        hasNoSignedAttributes: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder = __cordl_object
            .invoke("SetDirectSignature", (hasNoSignedAttributes))?;
        Ok(__cordl_ret)
    }
    pub fn WithSignedAttributeGenerator(
        &mut self,
        signedGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder = __cordl_object
            .invoke("WithSignedAttributeGenerator", (signedGen))?;
        Ok(__cordl_ret)
    }
    pub fn WithUnsignedAttributeGenerator(
        &mut self,
        unsignedGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder = __cordl_object
            .invoke("WithUnsignedAttributeGenerator", (unsignedGen))?;
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
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGeneratorBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
