#[cfg(feature = "Org+BouncyCastle+Cms+DefaultSignatureAlgorithmIdentifierFinder")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultSignatureAlgorithmIdentifierFinder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Cms+DefaultSignatureAlgorithmIdentifierFinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::DefaultSignatureAlgorithmIdentifierFinder =>
    "Org.BouncyCastle.Cms"."DefaultSignatureAlgorithmIdentifierFinder"
);
#[cfg(feature = "Org+BouncyCastle+Cms+DefaultSignatureAlgorithmIdentifierFinder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::DefaultSignatureAlgorithmIdentifierFinder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+DefaultSignatureAlgorithmIdentifierFinder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::DefaultSignatureAlgorithmIdentifierFinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+DefaultSignatureAlgorithmIdentifierFinder")]
impl crate::Org::BouncyCastle::Cms::DefaultSignatureAlgorithmIdentifierFinder {
    pub fn CreatePssParams(
        hashAlgId: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        saltSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::RsassaPssParameters,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::RsassaPssParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePssParams", (hashAlgId, saltSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn Find(
        &mut self,
        sigAlgName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        > = __cordl_object.invoke("Find", (sigAlgName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Generate(
        signatureAlgorithm: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Generate", (signatureAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+DefaultSignatureAlgorithmIdentifierFinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::DefaultSignatureAlgorithmIdentifierFinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
