#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEditorGameplaySceneSetupData {
    __cordl_parent: SceneSetupData,
    pub useFirstPersonFlyingController: bool,
    pub recordVRMovement: bool,
    pub playVRMovement: bool,
}
#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapEditorGameplaySceneSetupData => ""
    ."BeatmapEditorGameplaySceneSetupData"
);
#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
impl std::ops::Deref for BeatmapEditorGameplaySceneSetupData {
    type Target = SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
impl std::ops::DerefMut for BeatmapEditorGameplaySceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
impl BeatmapEditorGameplaySceneSetupData {
    pub fn New(
        useFirstPersonFlyingController: bool,
        recordVRMovement: bool,
        playVRMovement: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (useFirstPersonFlyingController, recordVRMovement, playVRMovement),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        useFirstPersonFlyingController: bool,
        recordVRMovement: bool,
        playVRMovement: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (useFirstPersonFlyingController, recordVRMovement, playVRMovement),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
impl quest_hook::libil2cpp::ObjectType for BeatmapEditorGameplaySceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}