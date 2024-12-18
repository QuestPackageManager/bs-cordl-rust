#[cfg(feature = "FakeMirrorObjectsInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct FakeMirrorObjectsInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _mirroredGameNoteControllerPrefab: *mut crate::GlobalNamespace::MirroredGameNoteController,
    pub _mirroredBurstSliderHeadGameNoteControllerPrefab: *mut crate::GlobalNamespace::MirroredGameNoteController,
    pub _mirroredBurstSliderGameNoteControllerPrefab: *mut crate::GlobalNamespace::MirroredGameNoteController,
    pub _mirroredBombNoteControllerPrefab: *mut crate::GlobalNamespace::MirroredBombNoteController,
    pub _mirroredObstacleControllerPrefab: *mut crate::GlobalNamespace::MirroredObstacleController,
    pub _mirroredSliderControllerPrefab: *mut crate::GlobalNamespace::MirroredSliderController,
    pub _mirrorRendererGraphicsSettingsPresets: *mut crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets,
    pub _graphicSettings: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FakeMirrorObjectsInstaller =>
    ""."FakeMirrorObjectsInstaller"
);
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::FakeMirrorObjectsInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::FakeMirrorObjectsInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl crate::GlobalNamespace::FakeMirrorObjectsInstaller {
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mirroredGameNoteControllerPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MirroredGameNoteController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MirroredGameNoteController,
        > = __cordl_object.invoke("get_mirroredGameNoteControllerPrefab", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FakeMirrorObjectsInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
