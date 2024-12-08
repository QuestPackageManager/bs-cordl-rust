#[cfg(feature = "Zenject+ZenjectManagersInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenjectManagersInstaller {
    __cordl_parent: crate::Zenject::Installer_1<
        *mut crate::Zenject::ZenjectManagersInstaller,
    >,
}
#[cfg(feature = "Zenject+ZenjectManagersInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ZenjectManagersInstaller => "Zenject"
    ."ZenjectManagersInstaller"
);
#[cfg(feature = "Zenject+ZenjectManagersInstaller")]
impl std::ops::Deref for crate::Zenject::ZenjectManagersInstaller {
    type Target = crate::Zenject::Installer_1<
        *mut crate::Zenject::ZenjectManagersInstaller,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectManagersInstaller")]
impl std::ops::DerefMut for crate::Zenject::ZenjectManagersInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectManagersInstaller")]
impl crate::Zenject::ZenjectManagersInstaller {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+ZenjectManagersInstaller")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ZenjectManagersInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
