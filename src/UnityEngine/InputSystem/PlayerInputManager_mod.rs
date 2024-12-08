#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerInputManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_NotificationBehavior: crate::UnityEngine::InputSystem::PlayerNotifications,
    pub m_MaxPlayerCount: i32,
    pub m_AllowJoining: bool,
    pub m_JoinBehavior: crate::UnityEngine::InputSystem::PlayerJoinBehavior,
    pub m_PlayerJoinedEvent: *mut crate::UnityEngine::InputSystem::PlayerInputManager_PlayerJoinedEvent,
    pub m_PlayerLeftEvent: *mut crate::UnityEngine::InputSystem::PlayerInputManager_PlayerLeftEvent,
    pub m_JoinAction: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_PlayerPrefab: *mut crate::UnityEngine::GameObject,
    pub m_SplitScreen: bool,
    pub m_MaintainAspectRatioInSplitScreen: bool,
    pub m_FixedNumberOfSplitScreens: i32,
    pub m_SplitScreenRect: crate::UnityEngine::Rect,
    pub m_JoinActionDelegateHooked: bool,
    pub m_UnpairedDeviceUsedDelegateHooked: bool,
    pub m_JoinActionDelegate: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_UnpairedDeviceUsedDelegate: *mut crate::System::Action_2<
        *mut crate::UnityEngine::InputSystem::InputControl,
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    >,
    pub m_PlayerJoinedCallbacks: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<*mut crate::UnityEngine::InputSystem::PlayerInput>,
    >,
    pub m_PlayerLeftCallbacks: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<*mut crate::UnityEngine::InputSystem::PlayerInput>,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::PlayerInputManager =>
    "UnityEngine.InputSystem"."PlayerInputManager"
);
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::PlayerInputManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::PlayerInputManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager")]
impl crate::UnityEngine::InputSystem::PlayerInputManager {
    pub const PlayerJoinedMessage: &'static str = "OnPlayerJoined";
    pub const PlayerLeftMessage: &'static str = "OnPlayerLeft";
    #[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerJoinedEvent")]
    pub type PlayerJoinedEvent = crate::UnityEngine::InputSystem::PlayerInputManager_PlayerJoinedEvent;
    #[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerLeftEvent")]
    pub type PlayerLeftEvent = crate::UnityEngine::InputSystem::PlayerInputManager_PlayerLeftEvent;
    pub fn CheckIfPlayerCanJoin(
        &mut self,
        playerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckIfPlayerCanJoin", (playerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn DisableJoining(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableJoining", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnableJoining(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableJoining", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsDeviceUsableWithPlayerActions(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsDeviceUsableWithPlayerActions", (device))?;
        Ok(__cordl_ret)
    }
    pub fn JoinPlayerFromAction(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("JoinPlayerFromAction", (context))?;
        Ok(__cordl_ret)
    }
    pub fn JoinPlayerFromActionIfNotAlreadyJoined(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("JoinPlayerFromActionIfNotAlreadyJoined", (context))?;
        Ok(__cordl_ret)
    }
    pub fn JoinPlayerFromUI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("JoinPlayerFromUI", ())?;
        Ok(__cordl_ret)
    }
    pub fn JoinPlayer_Il2CppArray1(
        &mut self,
        playerIndex: i32,
        splitScreenIndex: i32,
        controlScheme: *mut crate::System::String,
        pairWithDevices: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::PlayerInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::PlayerInput = __cordl_object
            .invoke(
                "JoinPlayer",
                (playerIndex, splitScreenIndex, controlScheme, pairWithDevices),
            )?;
        Ok(__cordl_ret)
    }
    pub fn JoinPlayer_InputDevice0(
        &mut self,
        playerIndex: i32,
        splitScreenIndex: i32,
        controlScheme: *mut crate::System::String,
        pairWithDevice: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::PlayerInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::PlayerInput = __cordl_object
            .invoke(
                "JoinPlayer",
                (playerIndex, splitScreenIndex, controlScheme, pairWithDevice),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn NotifyPlayerJoined(
        &mut self,
        player: *mut crate::UnityEngine::InputSystem::PlayerInput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyPlayerJoined", (player))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyPlayerLeft(
        &mut self,
        player: *mut crate::UnityEngine::InputSystem::PlayerInput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyPlayerLeft", (player))?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnUnpairedDeviceUsed(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnpairedDeviceUsed", (control, eventPtr))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSplitScreen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSplitScreen", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateInputActionAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateInputActionAsset", ())?;
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
    pub fn add_onPlayerJoined(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::PlayerInput,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onPlayerJoined", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onPlayerLeft(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::PlayerInput,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onPlayerLeft", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_fixedNumberOfSplitScreens(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_fixedNumberOfSplitScreens", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_joinAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_joinAction", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_joinBehavior(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::PlayerJoinBehavior,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::PlayerJoinBehavior = __cordl_object
            .invoke("get_joinBehavior", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_joiningEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_joiningEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maintainAspectRatioInSplitScreen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_maintainAspectRatioInSplitScreen", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxPlayerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxPlayerCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_notificationBehavior(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::PlayerNotifications,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::PlayerNotifications = __cordl_object
            .invoke("get_notificationBehavior", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_playerCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerJoinedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::PlayerInputManager_PlayerJoinedEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::PlayerInputManager_PlayerJoinedEvent = __cordl_object
            .invoke("get_playerJoinedEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerLeftEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::PlayerInputManager_PlayerLeftEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::PlayerInputManager_PlayerLeftEvent = __cordl_object
            .invoke("get_playerLeftEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_playerPrefab", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_splitScreen(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_splitScreen", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_splitScreenArea(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_splitScreenArea", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_onPlayerJoined(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::PlayerInput,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onPlayerJoined", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onPlayerLeft(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::PlayerInput,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onPlayerLeft", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_joinAction(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_joinAction", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_joinBehavior(
        &mut self,
        value: crate::UnityEngine::InputSystem::PlayerJoinBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_joinBehavior", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_notificationBehavior(
        &mut self,
        value: crate::UnityEngine::InputSystem::PlayerNotifications,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_notificationBehavior", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerPrefab(
        &mut self,
        value: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerPrefab", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_splitScreen(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_splitScreen", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::PlayerInputManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerJoinedEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerInputManager_PlayerJoinedEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        *mut crate::UnityEngine::InputSystem::PlayerInput,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerJoinedEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::PlayerInputManager_PlayerJoinedEvent =>
    "UnityEngine.InputSystem"."PlayerInputManager/PlayerJoinedEvent"
);
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerJoinedEvent")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::PlayerInputManager_PlayerJoinedEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        *mut crate::UnityEngine::InputSystem::PlayerInput,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerJoinedEvent")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::PlayerInputManager_PlayerJoinedEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerJoinedEvent")]
impl crate::UnityEngine::InputSystem::PlayerInputManager_PlayerJoinedEvent {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerJoinedEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::PlayerInputManager_PlayerJoinedEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerLeftEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerInputManager_PlayerLeftEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        *mut crate::UnityEngine::InputSystem::PlayerInput,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerLeftEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::PlayerInputManager_PlayerLeftEvent =>
    "UnityEngine.InputSystem"."PlayerInputManager/PlayerLeftEvent"
);
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerLeftEvent")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::PlayerInputManager_PlayerLeftEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        *mut crate::UnityEngine::InputSystem::PlayerInput,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerLeftEvent")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::PlayerInputManager_PlayerLeftEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerLeftEvent")]
impl crate::UnityEngine::InputSystem::PlayerInputManager_PlayerLeftEvent {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "UnityEngine+InputSystem+PlayerInputManager+PlayerLeftEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::PlayerInputManager_PlayerLeftEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
