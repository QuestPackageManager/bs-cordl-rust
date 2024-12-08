#[cfg(feature = "Org+BouncyCastle+Cms+BaseDigestCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseDigestCalculator {
    __cordl_parent: crate::System::Object,
    pub digest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Cms+BaseDigestCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::BaseDigestCalculator =>
    "Org.BouncyCastle.Cms"."BaseDigestCalculator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+BaseDigestCalculator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::BaseDigestCalculator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+BaseDigestCalculator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::BaseDigestCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+BaseDigestCalculator")]
impl crate::Org::BouncyCastle::Cms::BaseDigestCalculator {
    pub fn _ctor(
        &mut self,
        digest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest))?;
        Ok(__cordl_ret)
    }
    pub fn GetDigest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetDigest", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        digest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+BaseDigestCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::BaseDigestCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
