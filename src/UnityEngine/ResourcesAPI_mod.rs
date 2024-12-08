#[cfg(feature = "UnityEngine+ResourcesAPI")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourcesAPI {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ResourcesAPI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ResourcesAPI => "UnityEngine"
    ."ResourcesAPI"
);
#[cfg(feature = "UnityEngine+ResourcesAPI")]
impl std::ops::Deref for crate::UnityEngine::ResourcesAPI {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourcesAPI")]
impl std::ops::DerefMut for crate::UnityEngine::ResourcesAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourcesAPI")]
impl crate::UnityEngine::ResourcesAPI {
    pub fn FindObjectsOfTypeAll(
        &mut self,
        systemTypeInstance: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Object,
        > = __cordl_object.invoke("FindObjectsOfTypeAll", (systemTypeInstance))?;
        Ok(__cordl_ret)
    }
    pub fn FindShaderByName(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("FindShaderByName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn Load(
        &mut self,
        path: *mut crate::System::String,
        systemTypeInstance: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("Load", (path, systemTypeInstance))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAll(
        &mut self,
        path: *mut crate::System::String,
        systemTypeInstance: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Object,
        > = __cordl_object.invoke("LoadAll", (path, systemTypeInstance))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAsync(
        &mut self,
        path: *mut crate::System::String,
        systemTypeInstance: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::ResourceRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceRequest = __cordl_object
            .invoke("LoadAsync", (path, systemTypeInstance))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UnloadAsset(
        &mut self,
        assetToUnload: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadAsset", (assetToUnload))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ResourcesAPI")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ResourcesAPI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
