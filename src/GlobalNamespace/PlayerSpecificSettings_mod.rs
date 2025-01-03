#[cfg(feature = "PlayerSpecificSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSpecificSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _leftHanded: bool,
    pub _playerHeight: f32,
    pub _automaticPlayerHeight: bool,
    pub _sfxVolume: f32,
    pub _reduceDebris: bool,
    pub _noTextsAndHuds: bool,
    pub _noFailEffects: bool,
    pub _advancedHud: bool,
    pub _autoRestart: bool,
    pub _saberTrailIntensity: f32,
    pub _noteJumpDurationTypeSettings: crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    pub _noteJumpFixedDuration: f32,
    pub _noteJumpStartBeatOffset: f32,
    pub _hideNoteSpawnEffect: bool,
    pub _adaptiveSfx: bool,
    pub _arcsHapticFeedback: bool,
    pub _arcsVisible: crate::GlobalNamespace::ArcVisibilityType,
    pub _environmentEffectsFilterDefaultPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    pub _environmentEffectsFilterExpertPlusPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    pub _headsetHapticIntensity: f32,
}
#[cfg(feature = "PlayerSpecificSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerSpecificSettings => ""
    ."PlayerSpecificSettings"
);
#[cfg(feature = "PlayerSpecificSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSpecificSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSpecificSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSpecificSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSpecificSettings")]
impl crate::GlobalNamespace::PlayerSpecificSettings {
    pub fn AreValuesEqual(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreValuesEqual", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyWith(
        &mut self,
        leftHanded: crate::System::Nullable_1<bool>,
        playerHeight: crate::System::Nullable_1<f32>,
        automaticPlayerHeight: crate::System::Nullable_1<bool>,
        sfxVolume: crate::System::Nullable_1<f32>,
        reduceDebris: crate::System::Nullable_1<bool>,
        noTextsAndHuds: crate::System::Nullable_1<bool>,
        noFailEffects: crate::System::Nullable_1<bool>,
        advancedHud: crate::System::Nullable_1<bool>,
        autoRestart: crate::System::Nullable_1<bool>,
        saberTrailIntensity: crate::System::Nullable_1<f32>,
        noteJumpDurationTypeSettings: crate::System::Nullable_1<
            crate::GlobalNamespace::NoteJumpDurationTypeSettings,
        >,
        noteJumpFixedDuration: crate::System::Nullable_1<f32>,
        noteJumpStartBeatOffset: crate::System::Nullable_1<f32>,
        hideNoteSpawnEffect: crate::System::Nullable_1<bool>,
        adaptiveSfx: crate::System::Nullable_1<bool>,
        arcsHapticFeedback: crate::System::Nullable_1<bool>,
        arcsVisible: crate::System::Nullable_1<
            crate::GlobalNamespace::ArcVisibilityType,
        >,
        environmentEffectsFilterDefaultPreset: crate::System::Nullable_1<
            crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
        >,
        environmentEffectsFilterExpertPlusPreset: crate::System::Nullable_1<
            crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
        >,
        headsetHapticIntensity: crate::System::Nullable_1<f32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        > = __cordl_object
            .invoke(
                "CopyWith",
                (
                    leftHanded,
                    playerHeight,
                    automaticPlayerHeight,
                    sfxVolume,
                    reduceDebris,
                    noTextsAndHuds,
                    noFailEffects,
                    advancedHud,
                    autoRestart,
                    saberTrailIntensity,
                    noteJumpDurationTypeSettings,
                    noteJumpFixedDuration,
                    noteJumpStartBeatOffset,
                    hideNoteSpawnEffect,
                    adaptiveSfx,
                    arcsHapticFeedback,
                    arcsVisible,
                    environmentEffectsFilterDefaultPreset,
                    environmentEffectsFilterExpertPlusPreset,
                    headsetHapticIntensity,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentEffectsFilterPreset(
        &mut self,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentEffectsFilterPreset = __cordl_object
            .invoke("GetEnvironmentEffectsFilterPreset", (difficulty))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_f32__cordl_bool_f32__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool_f32_NoteJumpDurationTypeSettings_f32_f32__cordl_bool__cordl_bool__cordl_bool_ArcVisibilityType_EnvironmentEffectsFilterPreset_EnvironmentEffectsFilterPreset_f32_1(
        leftHanded: bool,
        playerHeight: f32,
        automaticPlayerHeight: bool,
        sfxVolume: f32,
        reduceDebris: bool,
        noTextsAndHuds: bool,
        noFailEffects: bool,
        advancedHud: bool,
        autoRestart: bool,
        saberTrailIntensity: f32,
        noteJumpDurationTypeSettings: crate::GlobalNamespace::NoteJumpDurationTypeSettings,
        noteJumpFixedDuration: f32,
        noteJumpStartBeatOffset: f32,
        hideNoteSpawnEffect: bool,
        adaptiveSfx: bool,
        arcsHapticFeedback: bool,
        arcsVisible: crate::GlobalNamespace::ArcVisibilityType,
        environmentEffectsFilterDefaultPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
        environmentEffectsFilterExpertPlusPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
        headsetHapticIntensity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    leftHanded,
                    playerHeight,
                    automaticPlayerHeight,
                    sfxVolume,
                    reduceDebris,
                    noTextsAndHuds,
                    noFailEffects,
                    advancedHud,
                    autoRestart,
                    saberTrailIntensity,
                    noteJumpDurationTypeSettings,
                    noteJumpFixedDuration,
                    noteJumpStartBeatOffset,
                    hideNoteSpawnEffect,
                    adaptiveSfx,
                    arcsHapticFeedback,
                    arcsVisible,
                    environmentEffectsFilterDefaultPreset,
                    environmentEffectsFilterExpertPlusPreset,
                    headsetHapticIntensity,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_f32__cordl_bool_f32__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool_f32_NoteJumpDurationTypeSettings_f32_f32__cordl_bool__cordl_bool__cordl_bool_ArcVisibilityType_EnvironmentEffectsFilterPreset_EnvironmentEffectsFilterPreset_f32_1(
        &mut self,
        leftHanded: bool,
        playerHeight: f32,
        automaticPlayerHeight: bool,
        sfxVolume: f32,
        reduceDebris: bool,
        noTextsAndHuds: bool,
        noFailEffects: bool,
        advancedHud: bool,
        autoRestart: bool,
        saberTrailIntensity: f32,
        noteJumpDurationTypeSettings: crate::GlobalNamespace::NoteJumpDurationTypeSettings,
        noteJumpFixedDuration: f32,
        noteJumpStartBeatOffset: f32,
        hideNoteSpawnEffect: bool,
        adaptiveSfx: bool,
        arcsHapticFeedback: bool,
        arcsVisible: crate::GlobalNamespace::ArcVisibilityType,
        environmentEffectsFilterDefaultPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
        environmentEffectsFilterExpertPlusPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
        headsetHapticIntensity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    leftHanded,
                    playerHeight,
                    automaticPlayerHeight,
                    sfxVolume,
                    reduceDebris,
                    noTextsAndHuds,
                    noFailEffects,
                    advancedHud,
                    autoRestart,
                    saberTrailIntensity,
                    noteJumpDurationTypeSettings,
                    noteJumpFixedDuration,
                    noteJumpStartBeatOffset,
                    hideNoteSpawnEffect,
                    adaptiveSfx,
                    arcsHapticFeedback,
                    arcsVisible,
                    environmentEffectsFilterDefaultPreset,
                    environmentEffectsFilterExpertPlusPreset,
                    headsetHapticIntensity,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_adaptiveSfx(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_adaptiveSfx", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_advancedHud(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_advancedHud", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_arcVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ArcVisibilityType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ArcVisibilityType = __cordl_object
            .invoke("get_arcVisibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_arcsHapticFeedback(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_arcsHapticFeedback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_autoRestart(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_autoRestart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_automaticPlayerHeight(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_automaticPlayerHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentEffectsFilterDefaultPreset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentEffectsFilterPreset = __cordl_object
            .invoke("get_environmentEffectsFilterDefaultPreset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentEffectsFilterExpertPlusPreset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentEffectsFilterPreset = __cordl_object
            .invoke("get_environmentEffectsFilterExpertPlusPreset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headsetHapticIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_headsetHapticIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hideNoteSpawnEffect(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hideNoteSpawnEffect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftHanded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_leftHanded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noFailEffects(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_noFailEffects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noTextsAndHuds(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_noTextsAndHuds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpDurationTypeSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteJumpDurationTypeSettings = __cordl_object
            .invoke("get_noteJumpDurationTypeSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpFixedDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteJumpFixedDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpStartBeatOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteJumpStartBeatOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_playerHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_reduceDebris(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_reduceDebris", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_saberTrailIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_saberTrailIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sfxVolume(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_sfxVolume", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerSpecificSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSpecificSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
