#[cfg(feature = "HealthWarningNoTransitionInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct HealthWarningNoTransitionInstaller {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::Zenject::NoTransitionInstaller>,
    pub _healthWarningSceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HealthWarningSceneSetupData,
    >,
    pub _scenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO,
    >,
}
#[cfg(feature = "HealthWarningNoTransitionInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::HealthWarningNoTransitionInstaller => ""
    ."HealthWarningNoTransitionInstaller"
);
#[cfg(feature = "HealthWarningNoTransitionInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::HealthWarningNoTransitionInstaller {
    type Target = quest_hook::libil2cpp::Gc<crate::Zenject::NoTransitionInstaller>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningNoTransitionInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::HealthWarningNoTransitionInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningNoTransitionInstaller")]
impl crate::GlobalNamespace::HealthWarningNoTransitionInstaller {
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
}
#[cfg(feature = "HealthWarningNoTransitionInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HealthWarningNoTransitionInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
