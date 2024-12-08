#[cfg(feature = "Org+BouncyCastle+Cms+CmsSecureReadable")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSecureReadable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSecureReadable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsSecureReadable =>
    "Org.BouncyCastle.Cms"."CmsSecureReadable"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSecureReadable")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsSecureReadable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSecureReadable")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsSecureReadable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSecureReadable")]
impl crate::Org::BouncyCastle::Cms::CmsSecureReadable {
    pub fn get_CryptoObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_CryptoObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Algorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_Algorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetReadable(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Cms::CmsReadable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsReadable = __cordl_object
            .invoke("GetReadable", (key))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSecureReadable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsSecureReadable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
