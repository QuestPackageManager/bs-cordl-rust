#[cfg(feature = "System+Net+Cache+RequestCachePolicy")]
#[repr(C)]
#[derive(Debug)]
pub struct RequestCachePolicy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Level: crate::System::Net::Cache::RequestCacheLevel,
}
#[cfg(feature = "System+Net+Cache+RequestCachePolicy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Cache::RequestCachePolicy =>
    "System.Net.Cache"."RequestCachePolicy"
);
#[cfg(feature = "System+Net+Cache+RequestCachePolicy")]
impl std::ops::Deref for crate::System::Net::Cache::RequestCachePolicy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCachePolicy")]
impl std::ops::DerefMut for crate::System::Net::Cache::RequestCachePolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCachePolicy")]
impl crate::System::Net::Cache::RequestCachePolicy {
    pub fn New(
        level: crate::System::Net::Cache::RequestCacheLevel,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (level))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        level: crate::System::Net::Cache::RequestCacheLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (level))?;
        Ok(__cordl_ret)
    }
    pub fn get_Level(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Cache::RequestCacheLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Cache::RequestCacheLevel = __cordl_object
            .invoke("get_Level", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Cache+RequestCachePolicy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Cache::RequestCachePolicy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
