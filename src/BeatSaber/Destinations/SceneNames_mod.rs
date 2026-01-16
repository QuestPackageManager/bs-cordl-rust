#[cfg(feature = "cordl_class_BeatSaber+Destinations+SceneNames")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneNames {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatSaber+Destinations+SceneNames")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Destinations::SceneNames {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Destinations";
    const CLASS_NAME: &'static str = "SceneNames";
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
#[cfg(feature = "BeatSaber+Destinations+SceneNames")]
impl std::ops::Deref for crate::BeatSaber::Destinations::SceneNames {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Destinations+SceneNames")]
impl std::ops::DerefMut for crate::BeatSaber::Destinations::SceneNames {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Destinations+SceneNames")]
impl crate::BeatSaber::Destinations::SceneNames {
    pub const kCreditsSceneName: &'static str = "Credits";
    pub const kEventsTestSceneName: &'static str = "EventsTest";
    pub const kGameCoreSceneName: &'static str = "GameCore";
    pub const kGameInitSceneName: &'static str = "GameInit";
    pub const kGameLoaderSceneName: &'static str = "GameLoader";
    pub const kHealthWarningSceneName: &'static str = "HealthWarning";
    pub const kMainMenuSceneName: &'static str = "MainMenu";
    pub const kMissionGameplaySceneName: &'static str = "MissionGameplay";
    pub const kMultiplayerEnvironmentSceneName: &'static str = "MultiplayerEnvironment";
    pub const kMultiplayerGameplaySceneName: &'static str = "MultiplayerGameplay";
    pub const kRecordingToolSceneName: &'static str = "RecordingTool";
    pub const kShaderWarmUpSceneName: &'static str = "ShaderWarmup";
    pub const kStandardGameplaySceneName: &'static str = "StandardGameplay";
    pub const kStartupErrorSceneName: &'static str = "StartupError";
    pub const kTheFirstEnvironmentSceneName: &'static str = "DefaultEnvironment";
    pub const kTutorialEnvironmentSceneName: &'static str = "TutorialEnvironment";
    pub const kTutorialSceneName: &'static str = "Tutorial";
    pub const kWaypointsTestSceneName: &'static str = "WaypointsTest";
    pub fn ShouldDisableRootObjects(
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        bool,
                        1usize,
                    >("ShouldDisableRootObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShouldDisableRootObjects", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (sceneName))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Destinations+SceneNames")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Destinations::SceneNames {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
