#[cfg(feature = "System+Net+Cache+RequestCacheBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct RequestCacheBinding {
    __cordl_parent: crate::System::Object,
    pub m_RequestCache: *mut crate::System::Net::Cache::RequestCache,
    pub m_CacheValidator: *mut crate::System::Net::Cache::RequestCacheValidator,
    pub m_Policy: *mut crate::System::Net::Cache::RequestCachePolicy,
}
#[cfg(feature = "System+Net+Cache+RequestCacheBinding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Cache::RequestCacheBinding =>
    "System.Net.Cache"."RequestCacheBinding"
);
#[cfg(feature = "System+Net+Cache+RequestCacheBinding")]
impl std::ops::Deref for crate::System::Net::Cache::RequestCacheBinding {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheBinding")]
impl std::ops::DerefMut for crate::System::Net::Cache::RequestCacheBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheBinding")]
impl crate::System::Net::Cache::RequestCacheBinding {
    pub fn New(
        requestCache: *mut crate::System::Net::Cache::RequestCache,
        cacheValidator: *mut crate::System::Net::Cache::RequestCacheValidator,
        policy: *mut crate::System::Net::Cache::RequestCachePolicy,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (requestCache, cacheValidator, policy))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        requestCache: *mut crate::System::Net::Cache::RequestCache,
        cacheValidator: *mut crate::System::Net::Cache::RequestCacheValidator,
        policy: *mut crate::System::Net::Cache::RequestCachePolicy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (requestCache, cacheValidator, policy))?;
        Ok(__cordl_ret)
    }
    pub fn get_Cache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Cache::RequestCache> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Cache::RequestCache = __cordl_object
            .invoke("get_Cache", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Policy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::Cache::RequestCachePolicy,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Cache::RequestCachePolicy = __cordl_object
            .invoke("get_Policy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Validator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::Cache::RequestCacheValidator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Cache::RequestCacheValidator = __cordl_object
            .invoke("get_Validator", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheBinding")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Cache::RequestCacheBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
