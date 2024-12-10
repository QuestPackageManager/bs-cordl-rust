#[cfg(feature = "StandardGameplaySceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardGameplaySceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub autoRestart: bool,
    pub beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
    pub backButtonText: *mut quest_hook::libil2cpp::Il2CppString,
    pub gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    pub startPaused: bool,
}
#[cfg(feature = "StandardGameplaySceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StandardGameplaySceneSetupData
    => ""."StandardGameplaySceneSetupData"
);
#[cfg(feature = "StandardGameplaySceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::StandardGameplaySceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardGameplaySceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandardGameplaySceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
