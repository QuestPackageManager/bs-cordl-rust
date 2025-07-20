#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGeneratorBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct SignerInfoGeneratorBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub directSignature: bool,
    pub signedGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    >,
    pub unsignedGen: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGeneratorBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "SignerInfoGeneratorBuilder";
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
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInfoGeneratorBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Build_Il2CppArray1(
        &mut self,
        signerFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
        subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::SignerInfoGenerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::ISignatureFactory,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cms::SignerInfoGenerator,
                >,
                2usize,
            >("Build")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as
                    quest_hook::libil2cpp::Type > ::class(), "Build", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInfoGenerator,
        > = unsafe {
            method.invoke_unchecked(self, (signerFactory, subjectKeyIdentifier))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Build_X509Certificate0(
        &mut self,
        contentSigner: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
        certificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::SignerInfoGenerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::ISignatureFactory,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::X509::X509Certificate,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cms::SignerInfoGenerator,
                >,
                2usize,
            >("Build")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as
                    quest_hook::libil2cpp::Type > ::class(), "Build", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInfoGenerator,
        > = unsafe { method.invoke_unchecked(self, (contentSigner, certificate))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateGenerator(
        &mut self,
        contentSigner: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        >,
        sigId: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::SignerInfoGenerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::ISignatureFactory,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cms::SignerInfoGenerator,
                >,
                2usize,
            >("CreateGenerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as
                    quest_hook::libil2cpp::Type > ::class(), "CreateGenerator", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInfoGenerator,
        > = unsafe { method.invoke_unchecked(self, (contentSigner, sigId))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetDirectSignature(
        &mut self,
        hasNoSignedAttributes: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
                >,
                1usize,
            >("SetDirectSignature")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as
                    quest_hook::libil2cpp::Type > ::class(), "SetDirectSignature", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
        > = unsafe { method.invoke_unchecked(self, (hasNoSignedAttributes))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithSignedAttributeGenerator(
        &mut self,
        signedGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
                >,
                1usize,
            >("WithSignedAttributeGenerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as
                    quest_hook::libil2cpp::Type > ::class(),
                    "WithSignedAttributeGenerator", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
        > = unsafe { method.invoke_unchecked(self, (signedGen))? };
        Ok(__cordl_ret.into())
    }
    pub fn WithUnsignedAttributeGenerator(
        &mut self,
        unsignedGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
                >,
                1usize,
            >("WithUnsignedAttributeGenerator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as
                    quest_hook::libil2cpp::Type > ::class(),
                    "WithUnsignedAttributeGenerator", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder,
        > = unsafe { method.invoke_unchecked(self, (unsignedGen))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Cms::SignerInfoGeneratorBuilder as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
