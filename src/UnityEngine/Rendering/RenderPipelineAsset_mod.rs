#[cfg(feature = "UnityEngine+Rendering+RenderPipelineAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderPipelineAsset {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::RenderPipelineAsset =>
    "UnityEngine.Rendering"."RenderPipelineAsset"
);
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineAsset")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RenderPipelineAsset {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineAsset")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::RenderPipelineAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineAsset")]
impl crate::UnityEngine::Rendering::RenderPipelineAsset {
    pub fn CreatePipeline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Rendering::RenderPipeline,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Rendering::RenderPipeline = __cordl_object
            .invoke("CreatePipeline", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalCreatePipeline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Rendering::RenderPipeline,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Rendering::RenderPipeline = __cordl_object
            .invoke("InternalCreatePipeline", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
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
    pub fn get_autodeskInteractiveMaskedShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_autodeskInteractiveMaskedShader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_autodeskInteractiveShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_autodeskInteractiveShader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_autodeskInteractiveTransparentShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_autodeskInteractiveTransparentShader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_default2DMaskMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_default2DMaskMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_default2DMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_default2DMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultLineMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_defaultLineMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_defaultMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultParticleMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_defaultParticleMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_defaultShader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultSpeedTree7Shader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_defaultSpeedTree7Shader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultSpeedTree8Shader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_defaultSpeedTree8Shader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultTerrainMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_defaultTerrainMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultUIETC1SupportedMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_defaultUIETC1SupportedMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultUIMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_defaultUIMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultUIOverdrawMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("get_defaultUIOverdrawMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_prefixedRenderingLayerMaskNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_prefixedRenderingLayerMaskNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_renderPipelineShaderTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_renderPipelineShaderTag", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_renderingLayerMaskNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_renderingLayerMaskNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_terrainDetailGrassBillboardShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_terrainDetailGrassBillboardShader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_terrainDetailGrassShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_terrainDetailGrassShader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_terrainDetailLitShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_terrainDetailLitShader", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderPipelineAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
