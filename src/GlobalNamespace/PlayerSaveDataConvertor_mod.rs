#[cfg(feature = "cordl_class_PlayerSaveDataConvertor")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataConvertor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_PlayerSaveDataConvertor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataConvertor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataConvertor";
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
#[cfg(feature = "PlayerSaveDataConvertor")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataConvertor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveDataConvertor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl crate::GlobalNamespace::PlayerSaveDataConvertor {
    pub fn GetRuntimeData_PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData2(
        saveData: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ArcVisibilityType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData),
                        crate::GlobalNamespace::ArcVisibilityType,
                        1usize,
                    >("GetRuntimeData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRuntimeData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::ArcVisibilityType = unsafe {
            cordl_method_info.invoke_unchecked((), (saveData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRuntimeData_PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData0(
        saveData: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData),
                        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                        1usize,
                    >("GetRuntimeData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRuntimeData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EnvironmentEffectsFilterPreset = unsafe {
            cordl_method_info.invoke_unchecked((), (saveData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRuntimeData_PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData1(
        saveData: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData),
                        crate::GlobalNamespace::NoteJumpDurationTypeSettings,
                        1usize,
                    >("GetRuntimeData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRuntimeData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::NoteJumpDurationTypeSettings = unsafe {
            cordl_method_info.invoke_unchecked((), (saveData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSaveData_ArcVisibilityType2(
        data: crate::GlobalNamespace::ArcVisibilityType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::ArcVisibilityType),
                        crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData,
                        1usize,
                    >("GetSaveData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSaveData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData = unsafe {
            cordl_method_info.invoke_unchecked((), (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSaveData_EnvironmentEffectsFilterPreset0(
        data: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::EnvironmentEffectsFilterPreset),
                        crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData,
                        1usize,
                    >("GetSaveData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSaveData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData = unsafe {
            cordl_method_info.invoke_unchecked((), (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSaveData_NoteJumpDurationTypeSettings1(
        data: crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::NoteJumpDurationTypeSettings),
                        crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData,
                        1usize,
                    >("GetSaveData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSaveData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData = unsafe {
            cordl_method_info.invoke_unchecked((), (data))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_PlayerSaveDataConvertor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
