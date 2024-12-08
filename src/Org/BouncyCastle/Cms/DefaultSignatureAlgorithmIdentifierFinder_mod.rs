#[cfg(feature = "Org+BouncyCastle+Cms+DefaultSignatureAlgorithmIdentifierFinder")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultSignatureAlgorithmIdentifierFinder {
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
    pub fn Find(
        &mut self,
        sigAlgName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("Find", (sigAlgName))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
