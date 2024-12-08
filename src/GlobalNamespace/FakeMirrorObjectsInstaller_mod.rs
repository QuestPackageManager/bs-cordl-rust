#[cfg(feature = "FakeMirrorObjectsInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct FakeMirrorObjectsInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _mirroredGameNoteControllerPrefab: *mut MirroredGameNoteController,
    pub _mirroredBurstSliderHeadGameNoteControllerPrefab: *mut MirroredGameNoteController,
    pub _mirroredBurstSliderGameNoteControllerPrefab: *mut MirroredGameNoteController,
    pub _mirroredBombNoteControllerPrefab: *mut MirroredBombNoteController,
    pub _mirroredObstacleControllerPrefab: *mut MirroredObstacleController,
    pub _mirroredSliderControllerPrefab: *mut MirroredSliderController,
    pub _mirrorRendererGraphicsSettingsPresets: *mut MirrorRendererGraphicsSettingsPresets,
    pub _graphicSettings: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FakeMirrorObjectsInstaller => ""
    ."FakeMirrorObjectsInstaller"
);
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl std::ops::Deref for FakeMirrorObjectsInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl std::ops::DerefMut for FakeMirrorObjectsInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl FakeMirrorObjectsInstaller {
    #[cfg(feature = "FakeMirrorObjectsInstaller+__c")]
    pub type __c = crate::GlobalNamespace::FakeMirrorObjectsInstaller___c;
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mirroredGameNoteControllerPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MirroredGameNoteController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MirroredGameNoteController = __cordl_object
            .invoke("get_mirroredGameNoteControllerPrefab", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl quest_hook::libil2cpp::ObjectType for FakeMirrorObjectsInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
