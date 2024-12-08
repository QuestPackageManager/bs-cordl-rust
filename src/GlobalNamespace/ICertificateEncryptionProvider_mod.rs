#[cfg(feature = "ICertificateEncryptionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ICertificateEncryptionProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ICertificateEncryptionProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ICertificateEncryptionProvider
    => ""."ICertificateEncryptionProvider"
);
#[cfg(feature = "ICertificateEncryptionProvider")]
impl std::ops::Deref for crate::GlobalNamespace::ICertificateEncryptionProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ICertificateEncryptionProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::ICertificateEncryptionProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ICertificateEncryptionProvider")]
impl crate::GlobalNamespace::ICertificateEncryptionProvider {
    pub fn SignData(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("SignData", (data, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ICertificateEncryptionProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ICertificateEncryptionProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
