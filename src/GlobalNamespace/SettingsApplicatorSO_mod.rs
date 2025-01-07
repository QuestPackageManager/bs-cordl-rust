#[cfg(feature = "SettingsApplicatorSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsApplicatorSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _mirrorRendererGraphicsSettingsPresets: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets,
    >,
    pub _mainEffectGraphicsSettingsPresets: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO,
    >,
    pub _bloomPrePassGraphicsSettingsPresets: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO,
    >,
    pub _mirrorRenderer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirrorRendererSO,
    >,
    pub _mainEffectContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainEffectContainerSO,
    >,
    pub _bloomPrePassEffectContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassEffectContainerSO,
    >,
    pub _hapticFeedbackManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HapticFeedbackManager,
    >,
    pub _audioManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioManagerSO>,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
    pub roomTransformOffsetDidUpdateEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
}
#[cfg(feature = "SettingsApplicatorSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SettingsApplicatorSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SettingsApplicatorSO";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn ApplyGameSettings(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyGameSettings", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyGraphicSettings(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        sceneType: crate::GlobalNamespace::SceneType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyGraphicSettings", (settings, sceneType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyWindowSettings(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::BeatSaber::Settings::WindowSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyWindowSettings", (settings))?;
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
