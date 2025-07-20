#[cfg(feature = "StandardGameplaySceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardGameplaySceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub autoRestart: bool,
    pub beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    pub backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiers,
    >,
    pub startPaused: bool,
}
#[cfg(feature = "StandardGameplaySceneSetupData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::StandardGameplaySceneSetupData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "StandardGameplaySceneSetupData";
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
#[cfg(feature = "StandardGameplaySceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::StandardGameplaySceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardGameplaySceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandardGameplaySceneSetupData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardGameplaySceneSetupData")]
impl crate::GlobalNamespace::StandardGameplaySceneSetupData {
    pub fn New(
        autoRestart: bool,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        startPaused: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    autoRestart,
                    beatmapKey,
                    beatmapLevel,
                    backButtonText,
                    gameplayModifiers,
                    startPaused,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        autoRestart: bool,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        startPaused: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::BeatmapKey,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        autoRestart,
                        beatmapKey,
                        beatmapLevel,
                        backButtonText,
                        gameplayModifiers,
                        startPaused,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardGameplaySceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardGameplaySceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
