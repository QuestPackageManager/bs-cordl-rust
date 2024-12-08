#[cfg(feature = "StandardLevelNoTransitionInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelNoTransitionInstaller {
    __cordl_parent: crate::Zenject::NoTransitionInstaller,
    pub _scenesTransitionSetupData: *mut StandardLevelScenesTransitionSetupDataSO,
    pub _standardLevelNoTransitionInstallerData: *mut StandardLevelNoTransitionInstallerData,
}
#[cfg(feature = "StandardLevelNoTransitionInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for StandardLevelNoTransitionInstaller => ""
    ."StandardLevelNoTransitionInstaller"
);
#[cfg(feature = "StandardLevelNoTransitionInstaller")]
impl std::ops::Deref for StandardLevelNoTransitionInstaller {
    type Target = crate::Zenject::NoTransitionInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstaller")]
impl std::ops::DerefMut for StandardLevelNoTransitionInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstaller")]
impl StandardLevelNoTransitionInstaller {
    pub fn InstallBindings(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", (container))?;
        Ok(__cordl_ret)
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
    pub fn get_standardLevelNoTransitionInstallerData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut StandardLevelNoTransitionInstallerData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut StandardLevelNoTransitionInstallerData = __cordl_object
            .invoke("get_standardLevelNoTransitionInstallerData", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstaller")]
impl quest_hook::libil2cpp::ObjectType for StandardLevelNoTransitionInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
