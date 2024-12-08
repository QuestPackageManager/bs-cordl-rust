#[cfg(feature = "BeatSaber+AvatarCore+AbstractAdapterInstallerSO")]
#[repr(C)]
#[derive(Debug)]
pub struct AbstractAdapterInstallerSO {
    __cordl_parent: crate::Zenject::ScriptableObjectInstaller,
}
#[cfg(feature = "BeatSaber+AvatarCore+AbstractAdapterInstallerSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::AbstractAdapterInstallerSO => "BeatSaber.AvatarCore"
    ."AbstractAdapterInstallerSO"
);
#[cfg(feature = "BeatSaber+AvatarCore+AbstractAdapterInstallerSO")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::AbstractAdapterInstallerSO {
    type Target = crate::Zenject::ScriptableObjectInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AbstractAdapterInstallerSO")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::AbstractAdapterInstallerSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AbstractAdapterInstallerSO")]
impl crate::BeatSaber::AvatarCore::AbstractAdapterInstallerSO {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AbstractAdapterInstallerSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::AbstractAdapterInstallerSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
