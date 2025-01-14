#[cfg(feature = "System+Net+Cache+RequestCacheBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct RequestCacheBinding {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_RequestCache: quest_hook::libil2cpp::Gc<
        crate::System::Net::Cache::RequestCache,
    >,
    pub m_CacheValidator: quest_hook::libil2cpp::Gc<
        crate::System::Net::Cache::RequestCacheValidator,
    >,
    pub m_Policy: quest_hook::libil2cpp::Gc<
        crate::System::Net::Cache::RequestCachePolicy,
    >,
}
#[cfg(feature = "System+Net+Cache+RequestCacheBinding")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Cache::RequestCacheBinding {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Cache";
    const CLASS_NAME: &'static str = "RequestCacheBinding";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Net+Cache+RequestCacheBinding")]
impl std::ops::Deref for crate::System::Net::Cache::RequestCacheBinding {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        requestCache: quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCache>,
        cacheValidator: quest_hook::libil2cpp::Gc<
            crate::System::Net::Cache::RequestCacheValidator,
        >,
        policy: quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCachePolicy>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (requestCache, cacheValidator, policy))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        requestCache: quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCache>,
        cacheValidator: quest_hook::libil2cpp::Gc<
            crate::System::Net::Cache::RequestCacheValidator,
        >,
        policy: quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCachePolicy>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCache>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Net::Cache::RequestCacheValidator,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Net::Cache::RequestCachePolicy,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (requestCache, cacheValidator, policy))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Cache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCache>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCache>,
                0usize,
            >("get_Cache")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Cache", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Cache::RequestCache,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Policy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCachePolicy>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCachePolicy>,
                0usize,
            >("get_Policy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Policy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Cache::RequestCachePolicy,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Validator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Cache::RequestCacheValidator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::Cache::RequestCacheValidator,
                >,
                0usize,
            >("get_Validator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Validator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Cache::RequestCacheValidator,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
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
