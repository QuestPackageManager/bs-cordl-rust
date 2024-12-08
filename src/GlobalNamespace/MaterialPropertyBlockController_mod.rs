#[cfg(feature = "MaterialPropertyBlockController")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialPropertyBlockController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _renderers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Renderer,
    >,
    pub _materialPropertyBlock: *mut crate::UnityEngine::MaterialPropertyBlock,
    pub _numberOfMaterialsInRenderers: *mut crate::System::Collections::Generic::List_1<
        i32,
    >,
    pub _isInitialized: bool,
}
#[cfg(feature = "MaterialPropertyBlockController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MaterialPropertyBlockController
    => ""."MaterialPropertyBlockController"
);
#[cfg(feature = "MaterialPropertyBlockController")]
impl std::ops::Deref for crate::GlobalNamespace::MaterialPropertyBlockController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyBlockController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MaterialPropertyBlockController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyBlockController")]
impl crate::GlobalNamespace::MaterialPropertyBlockController {
    pub fn ApplyChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyChanges", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetRendererState(
        &mut self,
        newState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRendererState", (newState))?;
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
    pub fn get_materialPropertyBlock(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::MaterialPropertyBlock> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::MaterialPropertyBlock = __cordl_object
            .invoke("get_materialPropertyBlock", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_renderers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Renderer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Renderer,
        > = __cordl_object.invoke("get_renderers", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MaterialPropertyBlockController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MaterialPropertyBlockController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
