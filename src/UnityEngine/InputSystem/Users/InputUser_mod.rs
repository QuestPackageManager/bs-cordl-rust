#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+CompareDevicesByUserAccount")]
#[repr(C)]
#[derive(Debug, Clone)]
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
        x: *mut crate::UnityEngine::InputSystem::InputDevice,
        y: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Compare",
            (x, y),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+ControlSchemeChangeSyntax")]
#[repr(C)]
#[derive(Debug, Clone)]
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+GlobalState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputUser_GlobalState {
    pub pairingStateVersion: i32,
    pub lastUserId: u32,
    pub allUserCount: i32,
    pub allPairedDeviceCount: i32,
    pub allLostDeviceCount: i32,
    pub allUsers: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::Users::InputUser,
    >,
    pub allUserData: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::Users::InputUser_UserData,
    >,
    pub allPairedDevices: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    >,
    pub allLostDevices: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    >,
    pub ongoingAccountSelections: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::Users::InputUser_OngoingAccountSelection,
    >,
    pub onChange: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_3<
            crate::UnityEngine::InputSystem::Users::InputUser,
            crate::UnityEngine::InputSystem::Users::InputUserChange,
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    >,
    pub onUnpairedDeviceUsed: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_2<
            *mut crate::UnityEngine::InputSystem::InputControl,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    >,
    pub onPreFilterUnpairedDeviceUsed: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Func_3<
            *mut crate::UnityEngine::InputSystem::InputDevice,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            bool,
        >,
    >,
    pub actionChangeDelegate: *mut crate::System::Action_2<
        *mut crate::System::Object,
        crate::UnityEngine::InputSystem::InputActionChange,
    >,
    pub onDeviceChangeDelegate: *mut crate::System::Action_2<
        *mut crate::UnityEngine::InputSystem::InputDevice,
        crate::UnityEngine::InputSystem::InputDeviceChange,
    >,
    pub onEventDelegate: *mut crate::System::Action_2<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        *mut crate::UnityEngine::InputSystem::InputDevice,
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
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser")]
#[repr(C)]
#[derive(Debug, Clone)]
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
    #[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+__c")]
    pub type __c = crate::UnityEngine::InputSystem::Users::InputUser___c;
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ActivateControlScheme_String0(
        &mut self,
        schemeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Users::InputUser_ControlSchemeChangeSyntax = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ActivateControlScheme",
            (schemeName),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AssociateActionsWithUser(
        &mut self,
        actions: *mut crate::UnityEngine::InputSystem::IInputActionCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AssociateActionsWithUser",
            (actions),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FindControlScheme(
        &mut self,
        schemeName: *mut crate::System::String,
        scheme: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FindControlScheme",
            (schemeName, scheme),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryFindControlScheme(
        &mut self,
        schemeName: *mut crate::System::String,
        scheme: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlScheme,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFindControlScheme",
            (schemeName, scheme),
        )?;
        Ok(__cordl_ret)
    }
    pub fn UnpairDevice(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnpairDevice",
            (device),
        )?;
        Ok(__cordl_ret)
    }
    pub fn UnpairDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnpairDevices",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn UnpairDevicesAndRemoveUser(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnpairDevicesAndRemoveUser",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_actions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::IInputActionCollection,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::IInputActionCollection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_actions",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_controlScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputControlScheme>,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputControlScheme,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_controlScheme", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_hasMissingRequiredDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hasMissingRequiredDevices",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_id",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_index",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_lostDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_lostDevices", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pairedDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_pairedDevices", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_platformUserAccountId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_platformUserAccountId",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_platformUserAccountName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_platformUserAccountName",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valid",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+OngoingAccountSelection")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputUser_OngoingAccountSelection {
    pub device: *mut crate::UnityEngine::InputSystem::InputDevice,
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
#[derive(Debug, Clone)]
pub struct InputUser_UserData {
    pub platformUserAccountHandle: crate::System::Nullable_1<
        crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
    >,
    pub platformUserAccountName: *mut crate::System::String,
    pub platformUserAccountId: *mut crate::System::String,
    pub deviceCount: i32,
    pub deviceStartIndex: i32,
    pub actions: *mut crate::UnityEngine::InputSystem::IInputActionCollection,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputUser_UserFlags {
    BindToAllDevices = 1i32,
    UserAccountSelectionInProgress = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUser+UserFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Users::InputUser_UserFlags =>
    "UnityEngine.InputSystem.Users"."InputUser/UserFlags"
);
