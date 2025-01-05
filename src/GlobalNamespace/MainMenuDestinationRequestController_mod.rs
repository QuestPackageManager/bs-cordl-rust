#[cfg(feature = "MainMenuDestinationRequestController")]
#[repr(C)]
#[derive(Debug)]
pub struct MainMenuDestinationRequestController {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _destinationRequestManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IDestinationRequestManager,
    >,
    pub _menuScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuScenesTransitionSetupDataSO,
    >,
    pub _gameScenesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameScenesManager,
    >,
    pub _cancellationTokenSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
}
#[cfg(feature = "MainMenuDestinationRequestController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MainMenuDestinationRequestController => ""
    ."MainMenuDestinationRequestController"
);
#[cfg(feature = "MainMenuDestinationRequestController")]
impl std::ops::Deref for crate::GlobalNamespace::MainMenuDestinationRequestController {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDestinationRequestManagerDidSendMenuDestinationRequest(
        &mut self,
        menuDestination: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuDestination,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleDestinationRequestManagerDidSendMenuDestinationRequest",
                (menuDestination),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameScenesManagerInstallEarlyBindings(
        &mut self,
        scenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleGameScenesManagerInstallEarlyBindings",
                (scenesTransitionSetupData, container),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessDestinationRequest(
        &mut self,
        menuDestination: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuDestination,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDestinationRequest", (menuDestination))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "MainMenuDestinationRequestController")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::MainMenuDestinationRequestController {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MainMenuDestinationRequestController")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::MainMenuDestinationRequestController {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MainMenuDestinationRequestController")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Zenject::IInitializable>>
for crate::GlobalNamespace::MainMenuDestinationRequestController {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::Zenject::IInitializable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MainMenuDestinationRequestController")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Zenject::IInitializable>>
for crate::GlobalNamespace::MainMenuDestinationRequestController {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Zenject::IInitializable> {
        unsafe { std::mem::transmute(self) }
    }
}
