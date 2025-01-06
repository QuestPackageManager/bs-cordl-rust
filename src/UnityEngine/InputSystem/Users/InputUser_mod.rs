#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputUser {
    pub m_Id: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Users::InputUser =>
    "UnityEngine.InputSystem.Users"."InputUser"
);
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Users::InputUser {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser")]
impl crate::UnityEngine::InputSystem::Users::InputUser {
    pub const InvalidId: u32 = 262656u32;
    #[cfg(
        feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount"
    )]
    pub type CompareDevicesByUserAccount = crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
    pub type ControlSchemeChangeSyntax = crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+GlobalState")]
    pub type GlobalState = crate::UnityEngine::InputSystem::Users::InputUser_GlobalState;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
    pub type OngoingAccountSelection = crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserData")]
    pub type UserData = crate::UnityEngine::InputSystem::Users::InputUser_UserData;
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserFlags")]
    pub type UserFlags = crate::UnityEngine::InputSystem::Users::InputUser_UserFlags;
    pub fn ActivateControlSchemeInternal(
        &mut self,
        userIndex: i32,
        scheme: crate::UnityEngine::InputSystem::InputControlScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ActivateControlSchemeInternal",
            (userIndex, scheme),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ActivateControlScheme_Il2CppString0(
        &mut self,
        schemeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ActivateControlScheme",
            (schemeName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ActivateControlScheme_InputControlScheme1(
        &mut self,
        scheme: crate::UnityEngine::InputSystem::InputControlScheme,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ActivateControlScheme",
            (scheme),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDeviceToUser(
        userIndex: i32,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        asLostDevice: bool,
        dontUpdateControlScheme: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddDeviceToUser",
                (userIndex, device, asLostDevice, dontUpdateControlScheme),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddUser() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddUser", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssociateActionsWithUser(
        &mut self,
        actions: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::IInputActionCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AssociateActionsWithUser",
            (actions),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUserWithoutPairedDevices() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Users::InputUser,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUserWithoutPairedDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisposeAndResetGlobalState() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisposeAndResetGlobalState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_InputUser0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Users::InputUser,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindControlScheme(
        &mut self,
        schemeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        scheme: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FindControlScheme",
            (schemeName, scheme),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FindLostDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindLostDevice", (device, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindUserByAccount(
        platformUserAccountHandle: crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::Users::InputUser>,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Users::InputUser,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindUserByAccount", (platformUserAccountHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindUserPairedToDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::Users::InputUser>,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Users::InputUser,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindUserPairedToDevice", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnpairedInputDevices_0() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlList_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlList_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnpairedInputDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnpairedInputDevices_ByRefMut1(
        list: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnpairedInputDevices", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn HookIntoActionChange() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HookIntoActionChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HookIntoDeviceChange() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HookIntoDeviceChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HookIntoEvents() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HookIntoEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitiateUserAccountSelection(
        userIndex: i32,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        options: crate::UnityEngine::InputSystem::Users::InputUserPairingOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitiateUserAccountSelection", (userIndex, device, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitiateUserAccountSelectionAtPlatformLevel(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitiateUserAccountSelectionAtPlatformLevel", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn Notify(
        userIndex: i32,
        change: crate::UnityEngine::InputSystem::Users::InputUserChange,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Notify", (userIndex, change, device))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnActionChange(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        change: crate::UnityEngine::InputSystem::InputActionChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnActionChange", (obj, change))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDeviceChange(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        change: crate::UnityEngine::InputSystem::InputDeviceChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnDeviceChange", (device, change))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEvent(
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnEvent", (eventPtr, device))?;
        Ok(__cordl_ret.into())
    }
    pub fn PerformPairingWithDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        user: crate::UnityEngine::InputSystem::Users::InputUser,
        options: crate::UnityEngine::InputSystem::Users::InputUserPairingOptions,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Users::InputUser,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PerformPairingWithDevice", (device, user, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueryPairedPlatformUserAccount(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        platformAccountHandle: quest_hook::libil2cpp::ByRefMut<
            crate::System::Nullable_1<
                crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
            >,
        >,
        platformAccountName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        platformAccountId: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "QueryPairedPlatformUserAccount",
                (device, platformAccountHandle, platformAccountName, platformAccountId),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDeviceFromUser(
        userIndex: i32,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        asLostDevice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveDeviceFromUser", (userIndex, device, asLostDevice))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveLostDevicesForUser(
        userIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveLostDevicesForUser", (userIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveUser(
        userIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveUser", (userIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetGlobals() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetGlobals", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAndResetState() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::ISavedState,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::ISavedState,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveAndResetState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindControlScheme(
        &mut self,
        schemeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        scheme: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFindControlScheme",
            (schemeName, scheme),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindUserIndex_InputDevice2(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindUserIndex", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindUserIndex_InputUserAccountHandle1(
        platformHandle: crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindUserIndex", (platformHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindUserIndex_u32_0(userId: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindUserIndex", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnhookFromActionChange() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnhookFromActionChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnhookFromDeviceChange() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnhookFromDeviceChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnhookFromDeviceStateChange() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnhookFromDeviceStateChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnpairDevice(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnpairDevice",
            (device),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnpairDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnpairDevices",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnpairDevicesAndRemoveUser(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnpairDevicesAndRemoveUser",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateControlSchemeMatch(
        userIndex: i32,
        autoPairMissing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateControlSchemeMatch", (userIndex, autoPairMissing))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePlatformUserAccount(
        userIndex: i32,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdatePlatformUserAccount", (userIndex, device))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                crate::UnityEngine::InputSystem::Users::InputUser,
                crate::UnityEngine::InputSystem::Users::InputUserChange,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onPrefilterUnpairedDeviceActivity(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onPrefilterUnpairedDeviceActivity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onUnpairedDeviceUsed(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onUnpairedDeviceUsed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_actions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::IInputActionCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::IInputActionCollection,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_actions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_all() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Users::InputUser,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::Users::InputUser,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_all", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controlScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputControlScheme>,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputControlScheme,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_controlScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controlSchemeMatch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme_MatchResult,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme_MatchResult = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_controlSchemeMatch",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasMissingRequiredDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hasMissingRequiredDevices",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_id",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_index",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_listenForUnpairedDeviceActivity() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_listenForUnpairedDeviceActivity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lostDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_lostDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pairedDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_pairedDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_platformUserAccountHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
        >,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_platformUserAccountHandle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_platformUserAccountId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_platformUserAccountId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_platformUserAccountName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_platformUserAccountName",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::UnityEngine::InputSystem::Users::InputUser,
        right: crate::UnityEngine::InputSystem::Users::InputUser,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::UnityEngine::InputSystem::Users::InputUser,
        right: crate::UnityEngine::InputSystem::Users::InputUser,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                crate::UnityEngine::InputSystem::Users::InputUser,
                crate::UnityEngine::InputSystem::Users::InputUserChange,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onPrefilterUnpairedDeviceActivity(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onPrefilterUnpairedDeviceActivity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onUnpairedDeviceUsed(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onUnpairedDeviceUsed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_listenForUnpairedDeviceActivity(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_listenForUnpairedDeviceActivity", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Users::InputUser>,
> for crate::UnityEngine::InputSystem::Users::InputUser {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Users::InputUser,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::InputSystem::Users::InputUser>,
> for crate::UnityEngine::InputSystem::Users::InputUser {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::Users::InputUser,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputUser_CompareDevicesByUserAccount {
    pub platformUserAccountHandle: crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount =>
    "UnityEngine.InputSystem.Users"."InputUser/CompareDevicesByUserAccount"
);
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
impl crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount {
    pub fn Compare(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        y: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Compare",
            (x, y),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserAccountHandleForDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
        >,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUserAccountHandleForDevice", (device))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
impl AsRef<
    crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    >,
> for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
impl AsMut<
    crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    >,
> for crate::UnityEngine::InputSystem::Users::InputUser_CompareDevicesByUserAccount {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputUser_ControlSchemeChangeSyntax {
    pub m_UserIndex: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax =>
    "UnityEngine.InputSystem.Users"."InputUser/ControlSchemeChangeSyntax"
);
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
impl crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax {
    pub fn AndPairRemainingDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AndPairRemainingDevices",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+GlobalState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputUser_GlobalState {
    pub pairingStateVersion: i32,
    pub lastUserId: u32,
    pub allUserCount: i32,
    pub allPairedDeviceCount: i32,
    pub allLostDeviceCount: i32,
    pub allUsers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Users::InputUser,
        >,
    >,
    pub allUserData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Users::InputUser_UserData,
        >,
    >,
    pub allPairedDevices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    >,
    pub allLostDevices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    >,
    pub ongoingAccountSelections: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection,
    >,
    pub onChange: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                crate::UnityEngine::InputSystem::Users::InputUser,
                crate::UnityEngine::InputSystem::Users::InputUserChange,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    >,
    pub onUnpairedDeviceUsed: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    >,
    pub onPreFilterUnpairedDeviceUsed: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                bool,
            >,
        >,
    >,
    pub actionChangeDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            crate::UnityEngine::InputSystem::InputActionChange,
        >,
    >,
    pub onDeviceChangeDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            crate::UnityEngine::InputSystem::InputDeviceChange,
        >,
    >,
    pub onEventDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    >,
    pub onActionChangeHooked: bool,
    pub onDeviceChangeHooked: bool,
    pub onEventHooked: bool,
    pub listenForUnpairedDeviceActivity: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+GlobalState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Users::InputUser_GlobalState =>
    "UnityEngine.InputSystem.Users"."InputUser/GlobalState"
);
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+GlobalState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Users::InputUser_GlobalState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+GlobalState")]
impl crate::UnityEngine::InputSystem::Users::InputUser_GlobalState {}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputUser_OngoingAccountSelection {
    pub device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    pub userId: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection =>
    "UnityEngine.InputSystem.Users"."InputUser/OngoingAccountSelection"
);
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
impl crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection {}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputUser_UserData {
    pub platformUserAccountHandle: crate::System::Nullable_1<
        crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
    >,
    pub platformUserAccountName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub platformUserAccountId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub deviceCount: i32,
    pub deviceStartIndex: i32,
    pub actions: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::IInputActionCollection,
    >,
    pub controlScheme: crate::System::Nullable_1<
        crate::UnityEngine::InputSystem::InputControlScheme,
    >,
    pub controlSchemeMatch: crate::UnityEngine::InputSystem::InputControlScheme_MatchResult,
    pub lostDeviceCount: i32,
    pub lostDeviceStartIndex: i32,
    pub flags: crate::UnityEngine::InputSystem::Users::InputUser_UserFlags,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Users::InputUser_UserData =>
    "UnityEngine.InputSystem.Users"."InputUser/UserData"
);
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Users::InputUser_UserData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserData")]
impl crate::UnityEngine::InputSystem::Users::InputUser_UserData {}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputUser_UserFlags {
    #[default]
    BindToAllDevices = 1i32,
    UserAccountSelectionInProgress = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Users::InputUser_UserFlags =>
    "UnityEngine.InputSystem.Users"."InputUser/UserFlags"
);
