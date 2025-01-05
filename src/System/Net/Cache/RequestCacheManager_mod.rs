#[cfg(feature = "System+Net+Cache+RequestCacheManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RequestCacheManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Net+Cache+RequestCacheManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Cache::RequestCacheManager =>
    "System.Net.Cache"."RequestCacheManager"
);
#[cfg(feature = "System+Net+Cache+RequestCacheManager")]
impl std::ops::Deref for crate::System::Net::Cache::RequestCacheManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
impl crate::System::Net::Cache::RequestCacheManager {
    pub fn GetBinding(
        internedScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCacheBinding>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Cache::RequestCacheBinding,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBinding", (internedScheme))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadConfigSettings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadConfigSettings", ())?;
        Ok(__cordl_ret.into())
    }
}
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
