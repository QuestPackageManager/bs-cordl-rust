#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCacheAllocationStrategy")]
#[repr(C)]
#[derive(Debug)]
pub struct LRUCacheAllocationStrategy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_poolMaxSize: i32,
    pub m_poolInitialCapacity: i32,
    pub m_poolCacheMaxSize: i32,
    pub m_poolCache: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    >,
    pub m_cache: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    >,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCacheAllocationStrategy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::LRUCacheAllocationStrategy =>
    "UnityEngine.ResourceManagement.Util"."LRUCacheAllocationStrategy"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCacheAllocationStrategy")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::LRUCacheAllocationStrategy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCacheAllocationStrategy")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::LRUCacheAllocationStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCacheAllocationStrategy")]
impl crate::UnityEngine::ResourceManagement::Util::LRUCacheAllocationStrategy {
    pub fn GetPool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetPool", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        typeHash: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("New", (_cordl_type, typeHash))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        poolMaxSize: i32,
        poolCapacity: i32,
        poolCacheMaxSize: i32,
        initialPoolCacheCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (poolMaxSize, poolCapacity, poolCacheMaxSize, initialPoolCacheCapacity),
            )?;
        Ok(__cordl_object)
    }
    pub fn Release(
        &mut self,
        typeHash: i32,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", (typeHash, obj))?;
        Ok(__cordl_ret)
    }
    pub fn ReleasePool(
        &mut self,
        pool: *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleasePool", (pool))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        poolMaxSize: i32,
        poolCapacity: i32,
        poolCacheMaxSize: i32,
        initialPoolCacheCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (poolMaxSize, poolCapacity, poolCacheMaxSize, initialPoolCacheCapacity),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCacheAllocationStrategy")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::LRUCacheAllocationStrategy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
