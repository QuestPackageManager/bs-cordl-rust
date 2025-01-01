#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllerProfileInstaller {
    __cordl_parent: crate::Zenject::Installer_1<
        *mut crate::BeatSaber::GameSettings::ControllerProfileInstaller,
    >,
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::GameSettings::ControllerProfileInstaller => "BeatSaber.GameSettings"
    ."ControllerProfileInstaller"
);
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileInstaller")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::ControllerProfileInstaller {
    type Target = crate::Zenject::Installer_1<
        *mut crate::BeatSaber::GameSettings::ControllerProfileInstaller,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileInstaller")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::ControllerProfileInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileInstaller")]
impl crate::BeatSaber::GameSettings::ControllerProfileInstaller {
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
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::ControllerProfileInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}