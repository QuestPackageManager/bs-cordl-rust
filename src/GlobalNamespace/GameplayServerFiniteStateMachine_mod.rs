#[cfg(feature = "GameplayServerFiniteStateMachine")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayServerFiniteStateMachine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _taskUtility_k__BackingField: *mut crate::BGNet::Core::ITaskUtility,
    pub _multiplayerSessionManager_k__BackingField: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _ownerUserId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _selectionMask_k__BackingField: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    pub _configuration_k__BackingField: crate::GlobalNamespace::GameplayServerConfiguration,
    pub _beatmapProvider_k__BackingField: *mut crate::GlobalNamespace::IServerBeatmapProvider,
    pub _menuRpcManager_k__BackingField: *mut crate::GlobalNamespace::MenuRpcManager,
    pub _gameplayRpcManager_k__BackingField: *mut crate::GlobalNamespace::GameplayRpcManager,
    pub state: *mut crate::GlobalNamespace::GameState,
    pub enteringState: bool,
}
#[cfg(feature = "GameplayServerFiniteStateMachine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayServerFiniteStateMachine => ""
    ."GameplayServerFiniteStateMachine"
);
#[cfg(feature = "GameplayServerFiniteStateMachine")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayServerFiniteStateMachine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachine")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayServerFiniteStateMachine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachine")]
impl crate::GlobalNamespace::GameplayServerFiniteStateMachine {
    #[cfg(feature = "GameplayServerFiniteStateMachine+InitParams")]
    pub type InitParams = crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams;
    pub fn New(
        initParams: crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initParams))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        initParams: crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IServerBeatmapProvider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IServerBeatmapProvider,
        > = __cordl_object.invoke("get_beatmapProvider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_configuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayServerConfiguration,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::GameplayServerConfiguration = __cordl_object
            .invoke("get_configuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayRpcManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayRpcManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayRpcManager,
        > = __cordl_object.invoke("get_gameplayRpcManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_menuRpcManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuRpcManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuRpcManager,
        > = __cordl_object.invoke("get_menuRpcManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplayerSessionManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMultiplayerSessionManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        > = __cordl_object.invoke("get_multiplayerSessionManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ownerUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ownerUserId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectionMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapLevelSelectionMask,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelSelectionMask = __cordl_object
            .invoke("get_selectionMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_taskUtility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility> = __cordl_object
            .invoke("get_taskUtility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapProvider(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IServerBeatmapProvider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapProvider", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_configuration(
        &mut self,
        value: crate::GlobalNamespace::GameplayServerConfiguration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_configuration", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gameplayRpcManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayRpcManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameplayRpcManager", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_menuRpcManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuRpcManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_menuRpcManager", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_multiplayerSessionManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multiplayerSessionManager", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ownerUserId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ownerUserId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectionMask(
        &mut self,
        value: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectionMask", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_taskUtility(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_taskUtility", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachine")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayServerFiniteStateMachine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachine+InitParams")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GameplayServerFiniteStateMachine_InitParams {
    pub taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    pub multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub creatorId: *mut quest_hook::libil2cpp::Il2CppString,
    pub selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    pub configuration: crate::GlobalNamespace::GameplayServerConfiguration,
    pub beatmapProvider: *mut crate::GlobalNamespace::IServerBeatmapProvider,
}
#[cfg(feature = "GameplayServerFiniteStateMachine+InitParams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams => ""
    ."GameplayServerFiniteStateMachine/InitParams"
);
#[cfg(feature = "GameplayServerFiniteStateMachine+InitParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachine+InitParams")]
impl crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams {
    pub fn _ctor(
        &mut self,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        creatorId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        configuration: crate::GlobalNamespace::GameplayServerConfiguration,
        beatmapProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IServerBeatmapProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                taskUtility,
                multiplayerSessionManager,
                creatorId,
                selectionMask,
                configuration,
                beatmapProvider,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
}
