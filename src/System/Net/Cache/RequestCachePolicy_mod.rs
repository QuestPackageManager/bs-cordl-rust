#[cfg(feature = "System+Net+Cache+RequestCachePolicy")]
#[repr(C)]
#[derive(Debug)]
pub struct RequestCachePolicy {
    __cordl_parent: crate::System::Object,
    pub m_Level: crate::System::Net::Cache::RequestCacheLevel,
}
#[cfg(feature = "System+Net+Cache+RequestCachePolicy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Cache::RequestCachePolicy =>
    "System.Net.Cache"."RequestCachePolicy"
);
#[cfg(feature = "System+Net+Cache+RequestCachePolicy")]
impl std::ops::Deref for crate::System::Net::Cache::RequestCachePolicy {
    type Target = crate::System::Object;
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
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
    pub fn New(
        level: crate::System::Net::Cache::RequestCacheLevel,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (level))?;
        Ok(__cordl_object)
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
