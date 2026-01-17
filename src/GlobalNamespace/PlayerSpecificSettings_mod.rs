#[cfg(feature = "cordl_class_PlayerSpecificSettings")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
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
    pub _environmentEffectsFilterDefaultPreset:
        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    pub _environmentEffectsFilterExpertPlusPreset:
        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    pub _headsetHapticIntensity: f32,
}
#[cfg(feature = "cordl_class_PlayerSpecificSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PlayerSpecificSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSpecificSettings";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "PlayerSpecificSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSpecificSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSpecificSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSpecificSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSpecificSettings")]
impl crate::GlobalNamespace::PlayerSpecificSettings {
    pub fn AreValuesEqual(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::PlayerSpecificSettings,
                        >),
                        bool,
                        1usize,
                    >("AreValuesEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AreValuesEqual", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
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
        arcsVisible: crate::System::Nullable_1<crate::GlobalNamespace::ArcVisibilityType>,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Nullable_1<bool>,
                            crate::System::Nullable_1<f32>,
                            crate::System::Nullable_1<bool>,
                            crate::System::Nullable_1<f32>,
                            crate::System::Nullable_1<bool>,
                            crate::System::Nullable_1<bool>,
                            crate::System::Nullable_1<bool>,
                            crate::System::Nullable_1<bool>,
                            crate::System::Nullable_1<bool>,
                            crate::System::Nullable_1<f32>,
                            crate::System::Nullable_1<
                                crate::GlobalNamespace::NoteJumpDurationTypeSettings,
                            >,
                            crate::System::Nullable_1<f32>,
                            crate::System::Nullable_1<f32>,
                            crate::System::Nullable_1<bool>,
                            crate::System::Nullable_1<bool>,
                            crate::System::Nullable_1<bool>,
                            crate::System::Nullable_1<
                                crate::GlobalNamespace::ArcVisibilityType,
                            >,
                            crate::System::Nullable_1<
                                crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                            >,
                            crate::System::Nullable_1<
                                crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                            >,
                            crate::System::Nullable_1<f32>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::PlayerSpecificSettings,
                        >,
                        20usize,
                    >("CopyWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyWith", 20usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings> = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
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
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentEffectsFilterPreset(
        &mut self,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentEffectsFilterPreset> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::BeatmapDifficulty),
                        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                        1usize,
                    >("GetEnvironmentEffectsFilterPreset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEnvironmentEffectsFilterPreset", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EnvironmentEffectsFilterPreset =
            unsafe { cordl_method_info.invoke_unchecked(self, (difficulty))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
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
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
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
    pub fn _ctor_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        bool,
                        f32,
                        bool,
                        f32,
                        bool,
                        bool,
                        bool,
                        bool,
                        bool,
                        f32,
                        crate::GlobalNamespace::NoteJumpDurationTypeSettings,
                        f32,
                        f32,
                        bool,
                        bool,
                        bool,
                        crate::GlobalNamespace::ArcVisibilityType,
                        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                        f32,
                    ), quest_hook::libil2cpp::Void, 20usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            20usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
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
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_adaptiveSfx(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_adaptiveSfx")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_adaptiveSfx",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_advancedHud(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_advancedHud")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_advancedHud",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_arcVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ArcVisibilityType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::ArcVisibilityType, 0usize>(
                        "get_arcVisibility",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_arcVisibility",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::ArcVisibilityType =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_arcsHapticFeedback(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_arcsHapticFeedback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_arcsHapticFeedback",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_autoRestart(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_autoRestart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_autoRestart",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_automaticPlayerHeight(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_automaticPlayerHeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_automaticPlayerHeight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentEffectsFilterDefaultPreset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentEffectsFilterPreset> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                        0usize,
                    >("get_environmentEffectsFilterDefaultPreset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_environmentEffectsFilterDefaultPreset", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EnvironmentEffectsFilterPreset =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentEffectsFilterExpertPlusPreset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentEffectsFilterPreset> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                        0usize,
                    >("get_environmentEffectsFilterExpertPlusPreset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_environmentEffectsFilterExpertPlusPreset", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EnvironmentEffectsFilterPreset =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_headsetHapticIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_headsetHapticIntensity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_headsetHapticIntensity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hideNoteSpawnEffect(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_hideNoteSpawnEffect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_hideNoteSpawnEffect",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_leftHanded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_leftHanded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_leftHanded",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_noFailEffects(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_noFailEffects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_noFailEffects",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_noTextsAndHuds(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_noTextsAndHuds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_noTextsAndHuds",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpDurationTypeSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteJumpDurationTypeSettings> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::NoteJumpDurationTypeSettings,
                        0usize,
                    >("get_noteJumpDurationTypeSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_noteJumpDurationTypeSettings", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteJumpDurationTypeSettings =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpFixedDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_noteJumpFixedDuration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_noteJumpFixedDuration",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpStartBeatOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_noteJumpStartBeatOffset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_noteJumpStartBeatOffset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_playerHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_playerHeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_playerHeight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_reduceDebris(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_reduceDebris")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_reduceDebris",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_saberTrailIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_saberTrailIntensity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_saberTrailIntensity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sfxVolume(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_sfxVolume")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_sfxVolume",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_PlayerSpecificSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlayerSpecificSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
