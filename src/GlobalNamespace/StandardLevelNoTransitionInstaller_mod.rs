#[cfg(feature = "StandardLevelNoTransitionInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelNoTransitionInstaller {
    __cordl_parent: crate::Zenject::NoTransitionInstaller,
    pub _scenesTransitionSetupData: *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    pub _standardLevelNoTransitionInstallerData: *mut crate::GlobalNamespace::StandardLevelNoTransitionInstallerData,
    pub _settingsManager: *mut crate::GlobalNamespace::SettingsManager,
}
#[cfg(feature = "StandardLevelNoTransitionInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelNoTransitionInstaller => ""
    ."StandardLevelNoTransitionInstaller"
);
#[cfg(feature = "StandardLevelNoTransitionInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelNoTransitionInstaller {
    type Target = crate::Zenject::NoTransitionInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandardLevelNoTransitionInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstaller")]
impl crate::GlobalNamespace::StandardLevelNoTransitionInstaller {
    pub fn InstallBindings(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", (container))?;
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
    pub fn get_standardLevelNoTransitionInstallerData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelNoTransitionInstallerData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelNoTransitionInstallerData,
        > = __cordl_object.invoke("get_standardLevelNoTransitionInstallerData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelNoTransitionInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
