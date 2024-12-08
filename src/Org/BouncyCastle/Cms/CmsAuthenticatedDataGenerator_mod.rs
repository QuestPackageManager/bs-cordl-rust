#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsAuthenticatedDataGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsAuthenticatedGenerator,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsAuthenticatedDataGenerator => "Org.BouncyCastle.Cms"
    ."CmsAuthenticatedDataGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataGenerator {
    type Target = crate::Org::BouncyCastle::Cms::CmsAuthenticatedGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataGenerator {
    pub fn Generate_CipherKeyGenerator0(
        &mut self,
        content: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        macOid: *mut crate::System::String,
        keyGen: *mut crate::Org::BouncyCastle::Crypto::CipherKeyGenerator,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsAuthenticatedData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsAuthenticatedData = __cordl_object
            .invoke("Generate", (content, macOid, keyGen))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_CmsProcessable_String1(
        &mut self,
        content: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        encryptionOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsAuthenticatedData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsAuthenticatedData = __cordl_object
            .invoke("Generate", (content, encryptionOid))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SecureRandom1(
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_rand))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SecureRandom1(
        &mut self,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_rand))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
