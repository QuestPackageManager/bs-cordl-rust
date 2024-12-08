#[cfg(feature = "Zenject+ActionInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct ActionInstaller {
    __cordl_parent: crate::Zenject::Installer_1<*mut crate::Zenject::ActionInstaller>,
    pub _installMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
}
#[cfg(feature = "Zenject+ActionInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ActionInstaller => "Zenject"
    ."ActionInstaller"
);
#[cfg(feature = "Zenject+ActionInstaller")]
impl std::ops::Deref for crate::Zenject::ActionInstaller {
    type Target = crate::Zenject::Installer_1<*mut crate::Zenject::ActionInstaller>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ActionInstaller")]
impl std::ops::DerefMut for crate::Zenject::ActionInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ActionInstaller")]
impl crate::Zenject::ActionInstaller {
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
        installMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (installMethod))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        installMethod: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (installMethod))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+ActionInstaller")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ActionInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
