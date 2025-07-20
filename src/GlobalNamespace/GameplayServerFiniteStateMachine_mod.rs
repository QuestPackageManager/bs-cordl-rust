#[cfg(feature = "GameplayServerFiniteStateMachine")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayServerFiniteStateMachine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _taskUtility_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BGNet::Core::ITaskUtility,
    >,
    pub _multiplayerSessionManager_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
    pub _ownerUserId_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _selectionMask_k__BackingField: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    pub _configuration_k__BackingField: crate::GlobalNamespace::GameplayServerConfiguration,
    pub _beatmapProvider_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IServerBeatmapProvider,
    >,
    pub _menuRpcManager_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuRpcManager,
    >,
    pub _gameplayRpcManager_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayRpcManager,
    >,
    pub state: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameState>,
    pub enteringState: bool,
}
#[cfg(feature = "GameplayServerFiniteStateMachine")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayServerFiniteStateMachine {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayServerFiniteStateMachine";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (initParams))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IServerBeatmapProvider>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IServerBeatmapProvider,
                >,
                0usize,
            >("get_beatmapProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "get_beatmapProvider",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IServerBeatmapProvider,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_configuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayServerConfiguration,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::GameplayServerConfiguration,
                0usize,
            >("get_configuration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "get_configuration", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::GameplayServerConfiguration = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayRpcManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayRpcManager>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayRpcManager>,
                0usize,
            >("get_gameplayRpcManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "get_gameplayRpcManager",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayRpcManager,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_menuRpcManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuRpcManager>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuRpcManager>,
                0usize,
            >("get_menuRpcManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "get_menuRpcManager", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MenuRpcManager,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplayerSessionManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IMultiplayerSessionManager>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IMultiplayerSessionManager,
                >,
                0usize,
            >("get_multiplayerSessionManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_multiplayerSessionManager", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ownerUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_ownerUserId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "get_ownerUserId", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_selectionMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapLevelSelectionMask,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::BeatmapLevelSelectionMask,
                0usize,
            >("get_selectionMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "get_selectionMask", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelSelectionMask = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_taskUtility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
                0usize,
            >("get_taskUtility")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "get_taskUtility", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapProvider(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IServerBeatmapProvider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IServerBeatmapProvider,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_beatmapProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "set_beatmapProvider",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_configuration(
        &mut self,
        value: crate::GlobalNamespace::GameplayServerConfiguration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::GameplayServerConfiguration),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_configuration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "set_configuration", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_gameplayRpcManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayRpcManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayRpcManager>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_gameplayRpcManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "set_gameplayRpcManager",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_menuRpcManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuRpcManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MenuRpcManager>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_menuRpcManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "set_menuRpcManager", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_multiplayerSessionManager(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IMultiplayerSessionManager,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_multiplayerSessionManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(),
                    "set_multiplayerSessionManager", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ownerUserId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ownerUserId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "set_ownerUserId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_selectionMask(
        &mut self,
        value: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::BeatmapLevelSelectionMask),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_selectionMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "set_selectionMask", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_taskUtility(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_taskUtility")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine as
                    quest_hook::libil2cpp::Type > ::class(), "set_taskUtility", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameplayServerFiniteStateMachine_InitParams {
    pub taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    pub multiplayerSessionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
    pub creatorId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    pub configuration: crate::GlobalNamespace::GameplayServerConfiguration,
    pub beatmapProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IServerBeatmapProvider,
    >,
}
#[cfg(feature = "GameplayServerFiniteStateMachine+InitParams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayServerFiniteStateMachine/InitParams";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachine+InitParams")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachine+InitParams")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachine+InitParams")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachine+InitParams")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IMultiplayerSessionManager,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::GlobalNamespace::BeatmapLevelSelectionMask,
                    crate::GlobalNamespace::GameplayServerConfiguration,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IServerBeatmapProvider,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        taskUtility,
                        multiplayerSessionManager,
                        creatorId,
                        selectionMask,
                        configuration,
                        beatmapProvider,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
