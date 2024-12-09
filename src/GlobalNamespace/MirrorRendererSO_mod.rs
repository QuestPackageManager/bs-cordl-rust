#[cfg(feature = "MirrorRendererSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MirrorRendererSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _reflectLayers: crate::UnityEngine::LayerMask,
    pub _stereoTextureWidth: i32,
    pub _stereoTextureHeight: i32,
    pub _monoTextureWidth: i32,
    pub _monoTextureHeight: i32,
    pub _maxAntiAliasing: i32,
    pub _disableDepthTexture: bool,
    pub _enableBloomPrePass: bool,
    pub _bloomPrePassRenderer: *mut crate::GlobalNamespace::BloomPrePassRendererSO,
    pub _bloomPrePassEffect: *mut crate::GlobalNamespace::BloomPrePassEffectSO,
    pub _clearDepthShader: *mut crate::UnityEngine::Shader,
    pub _bloomPrePassRenderTexture: *mut crate::UnityEngine::RenderTexture,
    pub _mirrorCamera: *mut crate::UnityEngine::Camera,
    pub _antialiasing: i32,
    pub _renderTextures: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
        *mut crate::UnityEngine::RenderTexture,
    >,
    pub kLeftRect: crate::UnityEngine::Rect,
    pub kRightRect: crate::UnityEngine::Rect,
    pub kFullRect: crate::UnityEngine::Rect,
}
#[cfg(feature = "MirrorRendererSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MirrorRendererSO => ""
    ."MirrorRendererSO"
);
#[cfg(feature = "MirrorRendererSO")]
impl std::ops::Deref for crate::GlobalNamespace::MirrorRendererSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MirrorRendererSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererSO")]
impl crate::GlobalNamespace::MirrorRendererSO {
    pub const kWaterLayer: i32 = 4i32;
    #[cfg(feature = "MirrorRendererSO+CameraTransformData")]
    pub type CameraTransformData = crate::GlobalNamespace::MirrorRendererSO_CameraTransformData;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateOrUpdateMirrorCamera(
        &mut self,
        currentCamera: *mut crate::UnityEngine::Camera,
        renderTexture: *mut crate::UnityEngine::RenderTexture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateOrUpdateMirrorCamera", (currentCamera, renderTexture))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        reflectLayers: crate::UnityEngine::LayerMask,
        stereoTextureWidth: i32,
        stereoTextureHeight: i32,
        monoTextureWidth: i32,
        monoTextureHeight: i32,
        maxAntiAliasing: i32,
        enableBloomPrePass: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    reflectLayers,
                    stereoTextureWidth,
                    stereoTextureHeight,
                    monoTextureWidth,
                    monoTextureHeight,
                    maxAntiAliasing,
                    enableBloomPrePass,
                ),
            )?;
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
    pub fn PrepareForNextFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareForNextFrame", ())?;
        Ok(__cordl_ret)
    }
    pub fn RenderMirror(
        &mut self,
        camPosition: crate::UnityEngine::Vector3,
        camRotation: crate::UnityEngine::Quaternion,
        camProjectionMatrix: crate::UnityEngine::Matrix4x4,
        screenRect: crate::UnityEngine::Rect,
        reclectionPlanePos: crate::UnityEngine::Vector3,
        reflectionPlaneNormal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RenderMirror",
                (
                    camPosition,
                    camRotation,
                    camProjectionMatrix,
                    screenRect,
                    reclectionPlanePos,
                    reflectionPlaneNormal,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RenderMirrorTexture(
        &mut self,
        reflectionPlanePos: crate::UnityEngine::Vector3,
        reflectionPlaneNormal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture = __cordl_object
            .invoke("RenderMirrorTexture", (reflectionPlanePos, reflectionPlaneNormal))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateParams", ())?;
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
#[cfg(feature = "MirrorRendererSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MirrorRendererSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MirrorRendererSO_CameraTransformData {
    pub position: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Quaternion,
    pub fov: f32,
    pub stereoEnabled: bool,
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MirrorRendererSO_CameraTransformData => ""
    ."MirrorRendererSO/CameraTransformData"
);
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MirrorRendererSO+CameraTransformData")]
impl crate::GlobalNamespace::MirrorRendererSO_CameraTransformData {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_MirrorRendererSO_CameraTransformData0(
        &mut self,
        other: crate::GlobalNamespace::MirrorRendererSO_CameraTransformData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
