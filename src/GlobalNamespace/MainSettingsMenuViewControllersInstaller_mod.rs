#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MainSettingsMenuViewControllersInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _defaultSettingsMenuViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainSettingsMenuViewController,
    >,
    pub _oculusPCSettingsMenuViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainSettingsMenuViewController,
    >,
    pub _questSettingsMenuViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainSettingsMenuViewController,
    >,
    pub _psvrSettingsMenuViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainSettingsMenuViewController,
    >,
    pub _psvr2SettingsMenuViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainSettingsMenuViewController,
    >,
    pub _tabBarViewControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TabBarViewController,
    >,
}
#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MainSettingsMenuViewControllersInstaller => ""
    ."MainSettingsMenuViewControllersInstaller"
);
#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
impl std::ops::Deref
for crate::GlobalNamespace::MainSettingsMenuViewControllersInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MainSettingsMenuViewControllersInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
impl crate::GlobalNamespace::MainSettingsMenuViewControllersInstaller {
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
#[cfg(feature = "MainSettingsMenuViewControllersInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MainSettingsMenuViewControllersInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
