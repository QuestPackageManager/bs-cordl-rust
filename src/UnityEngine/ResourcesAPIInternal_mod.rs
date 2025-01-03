#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourcesAPIInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ResourcesAPIInternal =>
    "UnityEngine"."ResourcesAPIInternal"
);
#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
impl std::ops::Deref for crate::UnityEngine::ResourcesAPIInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
impl std::ops::DerefMut for crate::UnityEngine::ResourcesAPIInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
impl crate::UnityEngine::ResourcesAPIInternal {
    pub fn FindObjectsOfTypeAll(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindObjectsOfTypeAll", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindShaderByName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindShaderByName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        systemTypeInstance: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Load", (path, systemTypeInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAll(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        systemTypeInstance: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAll", (path, systemTypeInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsyncInternal(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ResourceRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAsyncInternal", (path, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadAsset(
        assetToUnload: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadAsset", (assetToUnload))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourcesAPIInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ResourcesAPIInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
