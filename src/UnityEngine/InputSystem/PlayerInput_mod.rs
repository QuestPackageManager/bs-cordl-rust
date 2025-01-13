#[cfg(feature = "UnityEngine+InputSystem+PlayerInput")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerInput {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_Actions: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionAsset,
    >,
    pub m_NotificationBehavior: crate::UnityEngine::InputSystem::PlayerNotifications,
    pub m_UIInputModule: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule,
    >,
    pub m_DeviceLostEvent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::PlayerInput_DeviceLostEvent,
    >,
    pub m_DeviceRegainedEvent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::PlayerInput_DeviceRegainedEvent,
    >,
    pub m_ControlsChangedEvent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::PlayerInput_ControlsChangedEvent,
    >,
    pub m_ActionEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::PlayerInput_ActionEvent,
            >,
        >,
    >,
    pub m_NeverAutoSwitchControlSchemes: bool,
    pub m_DefaultControlScheme: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_DefaultActionMap: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_SplitScreenIndex: i32,
    pub m_Camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub m_InputValueObject: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputValue,
    >,
    pub m_CurrentActionMap: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionMap,
    >,
    pub m_PlayerIndex: i32,
    pub m_InputActive: bool,
    pub m_Enabled: bool,
    pub m_ActionsInitialized: bool,
    pub m_ActionMessageNames: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub m_InputUser: crate::UnityEngine::InputSystem::Users::InputUser,
    pub m_ActionTriggeredDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    >,
    pub m_DeviceLostCallbacks: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
            >,
        >,
    >,
    pub m_DeviceRegainedCallbacks: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
            >,
        >,
    >,
    pub m_ControlsChangedCallbacks: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
            >,
        >,
    >,
    pub m_ActionTriggeredCallbacks: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::InputAction_CallbackContext,
            >,
        >,
    >,
    pub m_UnpairedDeviceUsedDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    >,
    pub m_PreFilterUnpairedDeviceUsedDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            bool,
        >,
    >,
    pub m_OnUnpairedDeviceUsedHooked: bool,
    pub m_DeviceChangeDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            crate::UnityEngine::InputSystem::InputDeviceChange,
        >,
    >,
    pub m_OnDeviceChangeHooked: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::PlayerInput {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "PlayerInput";
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
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::PlayerInput {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::PlayerInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput")]
impl crate::UnityEngine::InputSystem::PlayerInput {
    pub const ControlsChangedMessage: &'static str = "OnControlsChanged";
    pub const DeviceLostMessage: &'static str = "OnDeviceLost";
    pub const DeviceRegainedMessage: &'static str = "OnDeviceRegained";
    #[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ActionEvent")]
    pub type ActionEvent = crate::UnityEngine::InputSystem::PlayerInput_ActionEvent;
    #[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ControlsChangedEvent")]
    pub type ControlsChangedEvent = crate::UnityEngine::InputSystem::PlayerInput_ControlsChangedEvent;
    #[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceLostEvent")]
    pub type DeviceLostEvent = crate::UnityEngine::InputSystem::PlayerInput_DeviceLostEvent;
    #[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceRegainedEvent")]
    pub type DeviceRegainedEvent = crate::UnityEngine::InputSystem::PlayerInput_DeviceRegainedEvent;
    pub fn ActivateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssignPlayerIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssignPlayerIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssignUserAndDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssignUserAndDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CacheMessageNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CacheMessageNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearCaches(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCaches", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DeactivateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DebugLogAction(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DebugLogAction", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoInstantiate(
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoInstantiate", (prefab))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindFirstPairedToDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindFirstPairedToDevice", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDevice<TDevice>(&mut self) -> quest_hook::libil2cpp::Result<TDevice>
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TDevice = __cordl_object.invoke("GetDevice", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerByIndex(
        playerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlayerByIndex", (playerIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleControlsChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleControlsChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDeviceLost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDeviceLost", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDeviceRegained(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDeviceRegained", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HaveBindingForDevice(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HaveBindingForDevice", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeActions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallOnActionTriggeredHook(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallOnActionTriggeredHook", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Il2CppArray1(
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        playerIndex: i32,
        controlScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        splitScreenIndex: i32,
        pairWithDevices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Instantiate",
                (prefab, playerIndex, controlScheme, splitScreenIndex, pairWithDevices),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_InputDevice0(
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        playerIndex: i32,
        controlScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        splitScreenIndex: i32,
        pairWithDevice: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Instantiate",
                (prefab, playerIndex, controlScheme, splitScreenIndex, pairWithDevice),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnActionTriggered(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnActionTriggered", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDeviceChange(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        change: crate::UnityEngine::InputSystem::InputDeviceChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeviceChange", (device, change))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPreFilterUnpairedDeviceUsed(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnPreFilterUnpairedDeviceUsed", (device, eventPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnUnpairedDeviceUsed(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnpairedDeviceUsed", (control, eventPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnUserChange(
        user: crate::UnityEngine::InputSystem::Users::InputUser,
        change: crate::UnityEngine::InputSystem::Users::InputUserChange,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnUserChange", (user, change, device))?;
        Ok(__cordl_ret.into())
    }
    pub fn PassivateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PassivateInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartListeningForDeviceChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartListeningForDeviceChanges", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartListeningForUnpairedDeviceActivity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartListeningForUnpairedDeviceActivity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopListeningForDeviceChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopListeningForDeviceChanges", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopListeningForUnpairedDeviceActivity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopListeningForUnpairedDeviceActivity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchControlSchemeInternal(
        &mut self,
        controlScheme: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlScheme,
        >,
        devices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchControlSchemeInternal", (controlScheme, devices))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchCurrentActionMap(
        &mut self,
        mapNameOrId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchCurrentActionMap", (mapNameOrId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchCurrentControlScheme_Il2CppArray0(
        &mut self,
        devices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SwitchCurrentControlScheme", (devices))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchCurrentControlScheme_Il2CppString_Il2CppArray1(
        &mut self,
        controlScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        devices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchCurrentControlScheme", (controlScheme, devices))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryToActivateControlScheme(
        &mut self,
        controlScheme: crate::UnityEngine::InputSystem::InputControlScheme,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryToActivateControlScheme", (controlScheme))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnassignUserAndDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnassignUserAndDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UninitializeActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UninitializeActions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UninstallOnActionTriggeredHook(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UninstallOnActionTriggeredHook", ())?;
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
    pub fn add_onActionTriggered(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::InputAction_CallbackContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onActionTriggered", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onControlsChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onControlsChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onDeviceLost(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onDeviceLost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onDeviceRegained(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onDeviceRegained", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_actionEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::PlayerInput_ActionEvent,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::PlayerInput_ActionEvent,
            >,
        > = __cordl_object.invoke("get_actionEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_actions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        > = __cordl_object.invoke("get_actions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_active(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_active", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_all() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_all", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_camera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = __cordl_object
            .invoke("get_camera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controlsChangedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput_ControlsChangedEvent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput_ControlsChangedEvent,
        > = __cordl_object.invoke("get_controlsChangedEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentActionMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        > = __cordl_object.invoke("get_currentActionMap", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentControlScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_currentControlScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultActionMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_defaultActionMap", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultControlScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_defaultControlScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceLostEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput_DeviceLostEvent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput_DeviceLostEvent,
        > = __cordl_object.invoke("get_deviceLostEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceRegainedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput_DeviceRegainedEvent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::PlayerInput_DeviceRegainedEvent,
        > = __cordl_object.invoke("get_deviceRegainedEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_devices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        > = __cordl_object.invoke("get_devices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasMissingRequiredDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasMissingRequiredDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inputIsActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_inputIsActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isSinglePlayer() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isSinglePlayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_neverAutoSwitchControlSchemes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_neverAutoSwitchControlSchemes", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_playerIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_playerIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_splitScreenIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_splitScreenIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uiInputModule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule,
        > = __cordl_object.invoke("get_uiInputModule", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_user(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Users::InputUser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser = __cordl_object
            .invoke("get_user", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onActionTriggered(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::InputAction_CallbackContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onActionTriggered", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onControlsChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onControlsChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onDeviceLost(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onDeviceLost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onDeviceRegained(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onDeviceRegained", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_actionEvents(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::PlayerInput_ActionEvent,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_actionEvents", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_actions(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_actions", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_camera(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_camera", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_currentActionMap(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentActionMap", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultActionMap(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultActionMap", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultControlScheme(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultControlScheme", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_neverAutoSwitchControlSchemes(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_neverAutoSwitchControlSchemes", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_uiInputModule(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uiInputModule", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InputSystem::PlayerInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ActionEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerInput_ActionEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_ActionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_ActionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ActionEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::PlayerInput_ActionEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "PlayerInput/ActionEvent";
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
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ActionEvent")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::PlayerInput_ActionEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ActionEvent")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::PlayerInput_ActionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ActionEvent")]
impl crate::UnityEngine::InputSystem::PlayerInput_ActionEvent {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Guid_Il2CppString2(
        actionGUID: crate::System::Guid,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (actionGUID, name))?;
        Ok(__cordl_object.into())
    }
    pub fn New_InputAction1(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Guid_Il2CppString2(
        &mut self,
        actionGUID: crate::System::Guid,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (actionGUID, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputAction1(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_actionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_actionId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_actionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_actionName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ActionEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::PlayerInput_ActionEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ControlsChangedEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerInput_ControlsChangedEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ControlsChangedEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::PlayerInput_ControlsChangedEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "PlayerInput/ControlsChangedEvent";
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
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ControlsChangedEvent")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::PlayerInput_ControlsChangedEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ControlsChangedEvent")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::PlayerInput_ControlsChangedEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ControlsChangedEvent")]
impl crate::UnityEngine::InputSystem::PlayerInput_ControlsChangedEvent {
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
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+ControlsChangedEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::PlayerInput_ControlsChangedEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceLostEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerInput_DeviceLostEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceLostEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::PlayerInput_DeviceLostEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "PlayerInput/DeviceLostEvent";
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
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceLostEvent")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::PlayerInput_DeviceLostEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceLostEvent")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::PlayerInput_DeviceLostEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceLostEvent")]
impl crate::UnityEngine::InputSystem::PlayerInput_DeviceLostEvent {
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
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceLostEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::PlayerInput_DeviceLostEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceRegainedEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerInput_DeviceRegainedEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceRegainedEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::PlayerInput_DeviceRegainedEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "PlayerInput/DeviceRegainedEvent";
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
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceRegainedEvent")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::PlayerInput_DeviceRegainedEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::PlayerInput>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceRegainedEvent")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::PlayerInput_DeviceRegainedEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceRegainedEvent")]
impl crate::UnityEngine::InputSystem::PlayerInput_DeviceRegainedEvent {
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
#[cfg(feature = "UnityEngine+InputSystem+PlayerInput+DeviceRegainedEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::PlayerInput_DeviceRegainedEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
