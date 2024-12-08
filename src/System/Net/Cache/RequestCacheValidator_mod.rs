#[cfg(feature = "System+Net+Cache+RequestCacheValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct RequestCacheValidator {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+Cache+RequestCacheValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Cache::RequestCacheValidator =>
    "System.Net.Cache"."RequestCacheValidator"
);
#[cfg(feature = "System+Net+Cache+RequestCacheValidator")]
impl std::ops::Deref for crate::System::Net::Cache::RequestCacheValidator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheValidator")]
impl std::ops::DerefMut for crate::System::Net::Cache::RequestCacheValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheValidator")]
impl crate::System::Net::Cache::RequestCacheValidator {
    pub fn CreateValidator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("CreateValidator", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Cache::RequestCacheValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
