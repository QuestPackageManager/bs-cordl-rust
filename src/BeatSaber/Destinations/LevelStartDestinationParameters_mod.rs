#[cfg(feature = "cordl_class_BeatSaber+Destinations+LevelStartDestinationParameters")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct LevelStartDestinationParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub gameMode: crate::BeatSaber::Destinations::GameMode,
    pub beatmapKey: quest_hook::libil2cpp::Gc<crate::BeatSaber::Destinations::SimpleBeatmapKey>,
    pub environmentOverride:
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Destinations::GameplayEnvironmentOverride>,
    pub gameplayModifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    pub customPlayerSpecificSettings:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    pub practiceSettings: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
    pub additionalInformation:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayAdditionalInformation>,
}
#[cfg(feature = "cordl_class_BeatSaber+Destinations+LevelStartDestinationParameters")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Destinations::LevelStartDestinationParameters
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Destinations";
    const CLASS_NAME: &'static str = "LevelStartDestinationParameters";
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
#[cfg(feature = "BeatSaber+Destinations+LevelStartDestinationParameters")]
impl std::ops::Deref for crate::BeatSaber::Destinations::LevelStartDestinationParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Destinations+LevelStartDestinationParameters")]
impl std::ops::DerefMut for crate::BeatSaber::Destinations::LevelStartDestinationParameters {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Destinations+LevelStartDestinationParameters")]
impl crate::BeatSaber::Destinations::LevelStartDestinationParameters {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_GameMode_SimpleBeatmapKey_GameplayEnvironmentOverride_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_GameplayAdditionalInformation1(
        gameMode: crate::BeatSaber::Destinations::GameMode,
        beatmapKey: quest_hook::libil2cpp::Gc<crate::BeatSaber::Destinations::SimpleBeatmapKey>,
        environmentOverride: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Destinations::GameplayEnvironmentOverride,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
        customPlayerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        practiceSettings: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
        additionalInformation: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayAdditionalInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                gameMode,
                beatmapKey,
                environmentOverride,
                gameplayModifiers,
                customPlayerSpecificSettings,
                practiceSettings,
                additionalInformation,
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
    pub fn _ctor_GameMode_SimpleBeatmapKey_GameplayEnvironmentOverride_GameplayModifiers_PlayerSpecificSettings_PracticeSettings_GameplayAdditionalInformation1(
        &mut self,
        gameMode: crate::BeatSaber::Destinations::GameMode,
        beatmapKey: quest_hook::libil2cpp::Gc<crate::BeatSaber::Destinations::SimpleBeatmapKey>,
        environmentOverride: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Destinations::GameplayEnvironmentOverride,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
        customPlayerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        practiceSettings: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
        additionalInformation: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayAdditionalInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::BeatSaber::Destinations::GameMode,
                        quest_hook::libil2cpp::Gc<crate::BeatSaber::Destinations::SimpleBeatmapKey>,
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Destinations::GameplayEnvironmentOverride,
                        >,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::GameplayAdditionalInformation,
                        >,
                    ), quest_hook::libil2cpp::Void, 7usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    gameMode,
                    beatmapKey,
                    environmentOverride,
                    gameplayModifiers,
                    customPlayerSpecificSettings,
                    practiceSettings,
                    additionalInformation,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Destinations+LevelStartDestinationParameters")]
impl quest_hook::libil2cpp::ObjectType
    for crate::BeatSaber::Destinations::LevelStartDestinationParameters
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
