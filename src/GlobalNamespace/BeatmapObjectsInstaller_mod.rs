#[cfg(feature = "BeatmapObjectsInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectsInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _normalBasicNotePrefab: *mut crate::GlobalNamespace::GameNoteController,
    pub _proModeNotePrefab: *mut crate::GlobalNamespace::GameNoteController,
    pub _burstSliderHeadNotePrefab: *mut crate::GlobalNamespace::GameNoteController,
    pub _burstSliderNotePrefab: *mut crate::GlobalNamespace::BurstSliderGameNoteController,
    pub _bombNotePrefab: *mut crate::GlobalNamespace::BombNoteController,
    pub _obstaclePrefab: *mut crate::GlobalNamespace::ObstacleController,
    pub _sliderShortPrefab: *mut crate::GlobalNamespace::SliderController,
    pub _sliderMediumPrefab: *mut crate::GlobalNamespace::SliderController,
    pub _sliderLongPrefab: *mut crate::GlobalNamespace::SliderController,
    pub _beatLinePrefab: *mut crate::GlobalNamespace::BeatLine,
    pub _sceneSetupData: *mut crate::GlobalNamespace::GameplayCoreSceneSetupData,
}
#[cfg(feature = "BeatmapObjectsInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapObjectsInstaller => ""
    ."BeatmapObjectsInstaller"
);
#[cfg(feature = "BeatmapObjectsInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectsInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectsInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInstaller")]
impl crate::GlobalNamespace::BeatmapObjectsInstaller {
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
}
#[cfg(feature = "BeatmapObjectsInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectsInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
