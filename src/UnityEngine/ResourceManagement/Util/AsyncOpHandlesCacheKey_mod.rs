#[cfg(feature = "UnityEngine+ResourceManagement+Util+AsyncOpHandlesCacheKey")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncOpHandlesCacheKey {
    __cordl_parent: crate::System::Object,
    pub m_Handles: *mut crate::System::Collections::Generic::HashSet_1<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    >,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+AsyncOpHandlesCacheKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::AsyncOpHandlesCacheKey =>
    "UnityEngine.ResourceManagement.Util"."AsyncOpHandlesCacheKey"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+AsyncOpHandlesCacheKey")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::AsyncOpHandlesCacheKey {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+AsyncOpHandlesCacheKey")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::AsyncOpHandlesCacheKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+AsyncOpHandlesCacheKey")]
impl crate::UnityEngine::ResourceManagement::Util::AsyncOpHandlesCacheKey {
    pub fn Equals_AsyncOpHandlesCacheKey2(
        &mut self,
        other: *mut crate::UnityEngine::ResourceManagement::Util::AsyncOpHandlesCacheKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_IOperationCacheKey1(
        &mut self,
        other: *mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        handles: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handles))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        handles: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handles))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+AsyncOpHandlesCacheKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::AsyncOpHandlesCacheKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
