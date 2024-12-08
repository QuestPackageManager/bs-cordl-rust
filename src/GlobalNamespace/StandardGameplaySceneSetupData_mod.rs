#[cfg(feature = "StandardGameplaySceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardGameplaySceneSetupData {
    __cordl_parent: SceneSetupData,
    pub autoRestart: bool,
    pub beatmapKey: BeatmapKey,
    pub beatmapLevel: *mut BeatmapLevel,
    pub backButtonText: *mut crate::System::String,
    pub gameplayModifiers: *mut GameplayModifiers,
    pub startPaused: bool,
}
#[cfg(feature = "StandardGameplaySceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for StandardGameplaySceneSetupData => ""
    ."StandardGameplaySceneSetupData"
);
#[cfg(feature = "StandardGameplaySceneSetupData")]
impl std::ops::Deref for StandardGameplaySceneSetupData {
    type Target = SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardGameplaySceneSetupData")]
impl std::ops::DerefMut for StandardGameplaySceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardGameplaySceneSetupData")]
impl StandardGameplaySceneSetupData {
    pub fn _ctor(
        &mut self,
        autoRestart: bool,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        backButtonText: *mut crate::System::String,
        gameplayModifiers: *mut GameplayModifiers,
        startPaused: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
        Ok(__cordl_ret)
    }
    pub fn New(
        autoRestart: bool,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        backButtonText: *mut crate::System::String,
        gameplayModifiers: *mut GameplayModifiers,
        startPaused: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
    }
}
#[cfg(feature = "StandardGameplaySceneSetupData")]
impl quest_hook::libil2cpp::ObjectType for StandardGameplaySceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
