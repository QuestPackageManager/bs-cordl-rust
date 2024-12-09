#[cfg(feature = "MainMenuDestinationRequestController")]
#[repr(C)]
#[derive(Debug)]
pub struct MainMenuDestinationRequestController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _destinationRequestManager: *mut crate::GlobalNamespace::IDestinationRequestManager,
    pub _menuScenesTransitionSetupData: *mut crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
    pub _gameScenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
}
#[cfg(feature = "MainMenuDestinationRequestController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MainMenuDestinationRequestController => ""
    ."MainMenuDestinationRequestController"
);
#[cfg(feature = "MainMenuDestinationRequestController")]
impl std::ops::Deref for crate::GlobalNamespace::MainMenuDestinationRequestController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainMenuDestinationRequestController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MainMenuDestinationRequestController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainMenuDestinationRequestController")]
impl crate::GlobalNamespace::MainMenuDestinationRequestController {
    #[cfg(
        feature = "MainMenuDestinationRequestController+_ProcessDestinationRequest_d__9"
    )]
    pub type _ProcessDestinationRequest_d__9 = crate::GlobalNamespace::MainMenuDestinationRequestController__ProcessDestinationRequest_d__9;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleDestinationRequestManagerDidSendMenuDestinationRequest(
        &mut self,
        menuDestination: *mut crate::GlobalNamespace::MenuDestination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleDestinationRequestManagerDidSendMenuDestinationRequest",
                (menuDestination),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleGameScenesManagerInstallEarlyBindings(
        &mut self,
        scenesTransitionSetupData: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleGameScenesManagerInstallEarlyBindings",
                (scenesTransitionSetupData, container),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessDestinationRequest(
        &mut self,
        menuDestination: *mut crate::GlobalNamespace::MenuDestination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDestinationRequest", (menuDestination))?;
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
}
#[cfg(feature = "MainMenuDestinationRequestController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MainMenuDestinationRequestController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
