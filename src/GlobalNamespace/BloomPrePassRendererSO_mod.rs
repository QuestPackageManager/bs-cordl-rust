#[cfg(feature = "BloomPrePassRendererSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassRendererSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _bloomFog: *mut crate::GlobalNamespace::BloomFogSO,
    pub _preallocationData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData,
    >,
    pub _lightsRenderingData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::GlobalNamespace::BloomPrePassLightTypeSO,
        *mut crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData,
    >,
    pub _commandBuffer: *mut crate::UnityEngine::Rendering::CommandBuffer,
    pub _initialized: bool,
    pub _blackTexture: *mut crate::UnityEngine::Texture2D,
    pub _lowestResBloomTexture: *mut crate::UnityEngine::RenderTexture,
}
#[cfg(feature = "BloomPrePassRendererSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomPrePassRendererSO => ""
    ."BloomPrePassRendererSO"
);
#[cfg(feature = "BloomPrePassRendererSO")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassRendererSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomPrePassRendererSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO")]
impl crate::GlobalNamespace::BloomPrePassRendererSO {
    #[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
    pub type LightsRenderingData = crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData;
    #[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
    pub type PreallocationData = crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData;
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateBloomPrePassRenderTextureIfNeeded(
        &mut self,
        renderTexture: *mut crate::UnityEngine::RenderTexture,
        bloomPrePassParams: *mut crate::GlobalNamespace::IBloomPrePassParams,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RenderTexture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RenderTexture = __cordl_object
            .invoke(
                "CreateBloomPrePassRenderTextureIfNeeded",
                (renderTexture, bloomPrePassParams),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DisableBloomFog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableBloomFog", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnableBloomFog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableBloomFog", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCameraParams(
        &mut self,
        camera: *mut crate::UnityEngine::Camera,
        projectionMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        stereoCameraEyeOffset: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetCameraParams",
                (camera, projectionMatrix, viewMatrix, stereoCameraEyeOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatrixLerp(
        &mut self,
        from: crate::UnityEngine::Matrix4x4,
        to: crate::UnityEngine::Matrix4x4,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("MatrixLerp", (from, to, t))?;
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
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn PrepareLightsMeshRendering(
        &mut self,
        lightType: *mut crate::GlobalNamespace::BloomPrePassLightTypeSO,
        data: *mut crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData,
        numberOfLights: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareLightsMeshRendering", (lightType, data, numberOfLights))?;
        Ok(__cordl_ret)
    }
    pub fn RenderAllLights(
        &mut self,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
        linesWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RenderAllLights", (viewMatrix, projectionMatrix, linesWidth))?;
        Ok(__cordl_ret)
    }
    pub fn RenderAndSetData(
        &mut self,
        cameraPos: crate::UnityEngine::Vector3,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        stereoCameraEyeOffset: f32,
        bloomPrePassParams: *mut crate::GlobalNamespace::IBloomPrePassParams,
        dest: *mut crate::UnityEngine::RenderTexture,
        textureToScreenRatio: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Vector2,
        >,
        toneMapping: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::ToneMapping>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RenderAndSetData",
                (
                    cameraPos,
                    projectionMatrix,
                    viewMatrix,
                    stereoCameraEyeOffset,
                    bloomPrePassParams,
                    dest,
                    textureToScreenRatio,
                    toneMapping,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetCustomStereoCameraEyeOffset(
        &mut self,
        stereoCameraEyeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCustomStereoCameraEyeOffset", (stereoCameraEyeOffset))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateBloomFogParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBloomFogParams", ())?;
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
#[cfg(feature = "BloomPrePassRendererSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassRendererSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassRendererSO_LightsRenderingData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mesh: *mut crate::UnityEngine::Mesh,
    pub lightQuads: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::BloomPrePassLight_QuadData,
    >,
    pub subMeshDescriptor: crate::UnityEngine::Rendering::SubMeshDescriptor,
}
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData => ""
    ."BloomPrePassRendererSO/LightsRenderingData"
);
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
impl crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "BloomPrePassRendererSO+LightsRenderingData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassRendererSO_LightsRenderingData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassRendererSO_PreallocationData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lightType: *mut crate::GlobalNamespace::BloomPrePassLightTypeSO,
    pub preallocateCount: i32,
}
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassRendererSO_PreallocationData => ""
    ."BloomPrePassRendererSO/PreallocationData"
);
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
impl crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "BloomPrePassRendererSO+PreallocationData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassRendererSO_PreallocationData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
