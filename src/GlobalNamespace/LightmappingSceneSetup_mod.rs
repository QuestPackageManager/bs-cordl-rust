#[cfg(feature = "LightmappingSceneSetup")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmappingSceneSetup {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _beatmapLevelSo: *mut crate::GlobalNamespace::BeatmapLevelSO,
    pub _beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    pub _beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub _colorScheme: *mut crate::GlobalNamespace::ColorSchemeSO,
    pub _standardLevelNoTransitionInstallerData: *mut crate::GlobalNamespace::StandardLevelNoTransitionInstallerData,
}
#[cfg(feature = "LightmappingSceneSetup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightmappingSceneSetup => ""
    ."LightmappingSceneSetup"
);
#[cfg(feature = "LightmappingSceneSetup")]
impl std::ops::Deref for crate::GlobalNamespace::LightmappingSceneSetup {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightmappingSceneSetup")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightmappingSceneSetup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightmappingSceneSetup")]
impl crate::GlobalNamespace::LightmappingSceneSetup {
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
    pub fn get_colorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::ColorSchemeSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ColorSchemeSO = __cordl_object
            .invoke("get_colorScheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_colorScheme(
        &mut self,
        value: *mut crate::GlobalNamespace::ColorSchemeSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorScheme", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LightmappingSceneSetup")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightmappingSceneSetup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
