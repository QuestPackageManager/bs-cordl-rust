#[cfg(feature = "MultiplayerLevelSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLevelSceneSetupData {
    __cordl_parent: SceneSetupData,
    pub beatmapKey: BeatmapKey,
    pub hasSong: bool,
}
#[cfg(feature = "MultiplayerLevelSceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLevelSceneSetupData => ""
    ."MultiplayerLevelSceneSetupData"
);
#[cfg(feature = "MultiplayerLevelSceneSetupData")]
impl std::ops::Deref for MultiplayerLevelSceneSetupData {
    type Target = SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelSceneSetupData")]
impl std::ops::DerefMut for MultiplayerLevelSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelSceneSetupData")]
impl MultiplayerLevelSceneSetupData {
    pub fn _ctor(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        hasSong: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapKey, hasSong))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        hasSong: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapKey, hasSong))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerLevelSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLevelSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
