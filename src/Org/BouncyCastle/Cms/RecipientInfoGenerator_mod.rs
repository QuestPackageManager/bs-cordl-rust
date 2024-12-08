#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct RecipientInfoGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInfoGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::RecipientInfoGenerator
    => "Org.BouncyCastle.Cms"."RecipientInfoGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInfoGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::RecipientInfoGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInfoGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::RecipientInfoGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInfoGenerator")]
impl crate::Org::BouncyCastle::Cms::RecipientInfoGenerator {
    pub fn Generate(
        &mut self,
        contentEncryptionKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo = __cordl_object
            .invoke("Generate", (contentEncryptionKey, random))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+RecipientInfoGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::RecipientInfoGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
