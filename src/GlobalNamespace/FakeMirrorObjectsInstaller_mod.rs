#[cfg(feature = "FakeMirrorObjectsInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct FakeMirrorObjectsInstaller {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::Zenject::MonoInstaller>,
    pub _mirroredGameNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredGameNoteController,
    >,
    pub _mirroredBurstSliderHeadGameNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredGameNoteController,
    >,
    pub _mirroredBurstSliderGameNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredGameNoteController,
    >,
    pub _mirroredBombNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredBombNoteController,
    >,
    pub _mirroredObstacleControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredObstacleController,
    >,
    pub _mirroredSliderControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredSliderController,
    >,
    pub _mirrorRendererGraphicsSettingsPresets: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets,
    >,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FakeMirrorObjectsInstaller =>
    ""."FakeMirrorObjectsInstaller"
);
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::FakeMirrorObjectsInstaller {
    type Target = quest_hook::libil2cpp::Gc<crate::Zenject::MonoInstaller>;
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
