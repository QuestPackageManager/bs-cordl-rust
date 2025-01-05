#[cfg(feature = "UnityEngine+ResourceManagement+Util+DependenciesCacheKey")]
#[repr(C)]
#[derive(Debug)]
pub struct DependenciesCacheKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Dependencies: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IList_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    >,
    pub m_DependenciesHash: i32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DependenciesCacheKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::DependenciesCacheKey =>
    "UnityEngine.ResourceManagement.Util"."DependenciesCacheKey"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DependenciesCacheKey")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::DependenciesCacheKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DependenciesCacheKey")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::DependenciesCacheKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DependenciesCacheKey")]
impl crate::UnityEngine::ResourceManagement::Util::DependenciesCacheKey {
    pub fn Equals_DependenciesCacheKey2(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::DependenciesCacheKey,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_IOperationCacheKey1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        dependencies: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        dependenciesHash: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dependencies, dependenciesHash))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        dependencies: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        dependenciesHash: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dependencies, dependenciesHash))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DependenciesCacheKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::DependenciesCacheKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DependenciesCacheKey")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    >,
> for crate::UnityEngine::ResourceManagement::Util::DependenciesCacheKey {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DependenciesCacheKey")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    >,
> for crate::UnityEngine::ResourceManagement::Util::DependenciesCacheKey {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DependenciesCacheKey")]
impl AsRef<crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey>
for crate::UnityEngine::ResourceManagement::Util::DependenciesCacheKey {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+DependenciesCacheKey")]
impl AsMut<crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey>
for crate::UnityEngine::ResourceManagement::Util::DependenciesCacheKey {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey {
        unsafe { std::mem::transmute(self) }
    }
}
