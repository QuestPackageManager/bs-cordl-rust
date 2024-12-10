#[cfg(feature = "SettingsApplicatorSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsApplicatorSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _mirrorRendererGraphicsSettingsPresets: *mut crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets,
    pub _mainEffectGraphicsSettingsPresets: *mut crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO,
    pub _bloomPrePassGraphicsSettingsPresets: *mut crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO,
    pub _mirrorRenderer: *mut crate::GlobalNamespace::MirrorRendererSO,
    pub _mainEffectContainer: *mut crate::GlobalNamespace::MainEffectContainerSO,
    pub _bloomPrePassEffectContainer: *mut crate::GlobalNamespace::BloomPrePassEffectContainerSO,
    pub _hapticFeedbackManager: *mut crate::GlobalNamespace::HapticFeedbackManager,
    pub _audioManager: *mut crate::GlobalNamespace::AudioManagerSO,
    pub _vrPlatformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
    pub roomTransformOffsetDidUpdateEvent: *mut crate::System::Action,
}
#[cfg(feature = "SettingsApplicatorSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SettingsApplicatorSO => ""
    ."SettingsApplicatorSO"
);
#[cfg(feature = "SettingsApplicatorSO")]
impl std::ops::Deref for crate::GlobalNamespace::SettingsApplicatorSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsApplicatorSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::SettingsApplicatorSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsApplicatorSO")]
impl crate::GlobalNamespace::SettingsApplicatorSO {
    pub fn ApplyMainSettings(
        &mut self,
        settings: quest_hook::libil2cpp::Gc<crate::BeatSaber::GameSettings::MainSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyMainSettings", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPerformancePreset(
        &mut self,
        preset: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::PerformancePresets::PerformancePreset,
        >,
        sceneType: crate::GlobalNamespace::SceneType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyPerformancePreset", (preset, sceneType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyWindowSettings(
        &mut self,
        resolution: crate::UnityEngine::Vector2Int,
        windowMode: crate::BeatSaber::GameSettings::WindowMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyWindowSettings", (resolution, windowMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyRoomTransformOffsetWasUpdated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyRoomTransformOffsetWasUpdated", ())?;
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
    pub fn add_roomTransformOffsetDidUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_roomTransformOffsetDidUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_roomTransformOffsetDidUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_roomTransformOffsetDidUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SettingsApplicatorSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SettingsApplicatorSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
