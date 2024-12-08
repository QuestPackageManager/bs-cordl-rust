#[cfg(feature = "System+Net+Cache+RequestCacheProtocol")]
#[repr(C)]
#[derive(Debug)]
pub struct RequestCacheProtocol {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+Cache+RequestCacheProtocol")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Cache::RequestCacheProtocol =>
    "System.Net.Cache"."RequestCacheProtocol"
);
#[cfg(feature = "System+Net+Cache+RequestCacheProtocol")]
impl std::ops::Deref for crate::System::Net::Cache::RequestCacheProtocol {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheProtocol")]
impl std::ops::DerefMut for crate::System::Net::Cache::RequestCacheProtocol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheProtocol")]
impl crate::System::Net::Cache::RequestCacheProtocol {
    pub fn New(
        arg1: *mut crate::System::Object,
        arg2: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (arg1, arg2))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        arg1: *mut crate::System::Object,
        arg2: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (arg1, arg2))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheProtocol")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Cache::RequestCacheProtocol {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}