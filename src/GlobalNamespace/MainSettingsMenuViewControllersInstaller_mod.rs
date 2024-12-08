#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MainSettingsMenuViewControllersInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _defaultSettingsMenuViewController: *mut MainSettingsMenuViewController,
    pub _oculusPCSettingsMenuViewController: *mut MainSettingsMenuViewController,
    pub _questSettingsMenuViewController: *mut MainSettingsMenuViewController,
    pub _psvrSettingsMenuViewController: *mut MainSettingsMenuViewController,
    pub _psvr2SettingsMenuViewController: *mut MainSettingsMenuViewController,
    pub _tabBarViewControllerPrefab: *mut TabBarViewController,
}
#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MainSettingsMenuViewControllersInstaller => ""
    ."MainSettingsMenuViewControllersInstaller"
);
#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
impl std::ops::Deref for MainSettingsMenuViewControllersInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
impl std::ops::DerefMut for MainSettingsMenuViewControllersInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
impl MainSettingsMenuViewControllersInstaller {
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
}
#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
impl quest_hook::libil2cpp::ObjectType for MainSettingsMenuViewControllersInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
