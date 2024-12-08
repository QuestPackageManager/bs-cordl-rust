#[cfg(feature = "Org+BouncyCastle+Cms+ISignerInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ISignerInfoGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Cms+ISignerInfoGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::ISignerInfoGenerator =>
    "Org.BouncyCastle.Cms"."ISignerInfoGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+ISignerInfoGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::ISignerInfoGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+ISignerInfoGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::ISignerInfoGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+ISignerInfoGenerator")]
impl crate::Org::BouncyCastle::Cms::ISignerInfoGenerator {
    pub fn Generate(
        &mut self,
        contentType: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        digestAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        calculatedDigest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo = __cordl_object
            .invoke("Generate", (contentType, digestAlgorithm, calculatedDigest))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+ISignerInfoGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::ISignerInfoGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}