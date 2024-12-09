#[cfg(feature = "System+Net+Cache+RequestCache")]
#[repr(C)]
#[derive(Debug)]
pub struct RequestCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Cache+RequestCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Cache::RequestCache =>
    "System.Net.Cache"."RequestCache"
);
#[cfg(feature = "System+Net+Cache+RequestCache")]
impl std::ops::Deref for crate::System::Net::Cache::RequestCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCache")]
impl std::ops::DerefMut for crate::System::Net::Cache::RequestCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCache")]
impl crate::System::Net::Cache::RequestCache {}
#[cfg(feature = "System+Net+Cache+RequestCache")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Cache::RequestCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
