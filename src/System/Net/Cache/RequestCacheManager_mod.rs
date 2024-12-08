#[cfg(feature = "System+Net+Cache+RequestCacheManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RequestCacheManager {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+Cache+RequestCacheManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Cache::RequestCacheManager =>
    "System.Net.Cache"."RequestCacheManager"
);
#[cfg(feature = "System+Net+Cache+RequestCacheManager")]
impl std::ops::Deref for crate::System::Net::Cache::RequestCacheManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheManager")]
impl std::ops::DerefMut for crate::System::Net::Cache::RequestCacheManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheManager")]
impl crate::System::Net::Cache::RequestCacheManager {}
#[cfg(feature = "System+Net+Cache+RequestCacheManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Cache::RequestCacheManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
