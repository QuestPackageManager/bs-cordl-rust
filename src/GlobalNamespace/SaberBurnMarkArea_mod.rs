#[cfg(feature = "SaberBurnMarkArea")]
#[repr(C)]
#[derive(Debug)]
pub struct SaberBurnMarkArea {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _saberBurnMarkLinePrefab: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::LineRenderer,
    >,
    pub _blackMarkLineRandomOffset: f32,
    pub _textureWidth: i32,
    pub _textureHeight: i32,
    pub _burnMarksFadeOutStrength: f32,
    pub _fadeOutShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub _colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
    pub _saberManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SaberManager>,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _disableBlitTimer: f32,
    pub _renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
    pub _fadeOutStrengthShaderPropertyID: i32,
    pub _sabers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::Saber>,
    >,
    pub _plane: crate::UnityEngine::Plane,
    pub _prevBurnMarkPos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub _prevBurnMarkPosValid: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<bool>,
    >,
    pub _lineRenderers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::LineRenderer>,
    >,
    pub _camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _linePoints: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub _renderTextures: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::RenderTexture>,
    >,
    pub _emitParams: crate::UnityEngine::ParticleSystem_EmitParams,
    pub _fadeOutMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _random: quest_hook::libil2cpp::Gc<crate::System::Random>,
}
#[cfg(feature = "SaberBurnMarkArea")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SaberBurnMarkArea => ""
    ."SaberBurnMarkArea"
);
#[cfg(feature = "SaberBurnMarkArea")]
impl std::ops::Deref for crate::GlobalNamespace::SaberBurnMarkArea {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SaberBurnMarkArea")]
impl std::ops::DerefMut for crate::GlobalNamespace::SaberBurnMarkArea {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SaberBurnMarkArea")]
impl crate::GlobalNamespace::SaberBurnMarkArea {
    pub const kDisableBlitAfterSecondsThreshold: f32 = 5f32;
    pub fn GetBurnMarkPos(
        &mut self,
        bladeBottomPos: crate::UnityEngine::Vector3,
        bladeTopPos: crate::UnityEngine::Vector3,
        burnMarkPos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetBurnMarkPos", (bladeBottomPos, bladeTopPos, burnMarkPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldToCameraBurnMarkPos(
        &mut self,
        pos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("WorldToCameraBurnMarkPos", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SaberBurnMarkArea")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SaberBurnMarkArea {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
