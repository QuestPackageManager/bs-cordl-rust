#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+SchemeJson+DeviceJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SchemeJson_DeviceJson {
    pub devicePath: *mut crate::System::String,
    pub isOptional: bool,
    pub isOR: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+SchemeJson+DeviceJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::SchemeJson_DeviceJson
    => "UnityEngine.InputSystem"."InputControlScheme/SchemeJson/DeviceJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+SchemeJson+DeviceJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::SchemeJson_DeviceJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+SchemeJson+DeviceJson")]
impl crate::UnityEngine::InputSystem::SchemeJson_DeviceJson {
    pub fn ToDeviceEntry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToDeviceEntry",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+DeviceRequirement")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputControlScheme_DeviceRequirement {
    pub m_ControlPath: *mut crate::System::String,
    pub m_Flags: crate::UnityEngine::InputSystem::DeviceRequirement_Flags,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+DeviceRequirement")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement =>
    "UnityEngine.InputSystem"."InputControlScheme/DeviceRequirement"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+DeviceRequirement")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+DeviceRequirement")]
impl crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement {
    #[cfg(
        feature = "UnityEngine+InputSystem+InputControlScheme+DeviceRequirement+Flags"
    )]
    pub type Flags = crate::UnityEngine::InputSystem::DeviceRequirement_Flags;
    pub fn Equals_InputControlScheme_DeviceRequirement0(
        &mut self,
        other: crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement,
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
    pub fn get_controlPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_controlPath",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isAND(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isAND",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isOR(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isOR",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isOptional(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isOptional",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_controlPath(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_controlPath",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isAND(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isAND",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isOR(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isOR",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isOptional(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isOptional",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MatchResult_Enumerator {
    pub m_Index: i32,
    pub m_Requirements: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement,
    >,
    pub m_Controls: crate::UnityEngine::InputSystem::InputControlList_1<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::MatchResult_Enumerator
    => "UnityEngine.InputSystem"."InputControlScheme/MatchResult/Enumerator"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Enumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::MatchResult_Enumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Enumerator")]
impl crate::UnityEngine::InputSystem::MatchResult_Enumerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::MatchResult_Match,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::MatchResult_Match = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+DeviceRequirement+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceRequirement_Flags {
    None = 0i32,
    Optional = 1i32,
    Or = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+DeviceRequirement+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::DeviceRequirement_Flags => "UnityEngine.InputSystem"
    ."InputControlScheme/DeviceRequirement/Flags"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputControlScheme {
    pub m_Name: *mut crate::System::String,
    pub m_BindingGroup: *mut crate::System::String,
    pub m_DeviceRequirements: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputControlScheme =>
    "UnityEngine.InputSystem"."InputControlScheme"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlScheme {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme")]
impl crate::UnityEngine::InputSystem::InputControlScheme {
    #[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+DeviceRequirement")]
    pub type DeviceRequirement = crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult")]
    pub type MatchResult = crate::UnityEngine::InputSystem::InputControlScheme_MatchResult;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+SchemeJson")]
    pub type SchemeJson = crate::UnityEngine::InputSystem::InputControlScheme_SchemeJson;
    pub fn Equals_InputControlScheme0(
        &mut self,
        other: crate::UnityEngine::InputSystem::InputControlScheme,
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn PickDevicesFrom<TDevices>(
        &mut self,
        devices: TDevices,
        favorDevice: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme_MatchResult,
    >
    where
        TDevices: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme_MatchResult = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PickDevicesFrom",
            (devices, favorDevice),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetNameAndBindingGroup(
        &mut self,
        name: *mut crate::System::String,
        bindingGroup: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetNameAndBindingGroup",
            (name, bindingGroup),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SupportsDevice(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SupportsDevice",
            (device),
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
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        devices: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement,
        >,
        bindingGroup: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name, devices, bindingGroup),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_bindingGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bindingGroup",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_deviceRequirements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deviceRequirements",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_name",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_bindingGroup(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bindingGroup",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Match")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MatchResult_Match {
    pub m_RequirementIndex: i32,
    pub m_Requirements: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement,
    >,
    pub m_Controls: crate::UnityEngine::InputSystem::InputControlList_1<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Match")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::MatchResult_Match =>
    "UnityEngine.InputSystem"."InputControlScheme/MatchResult/Match"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Match")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::MatchResult_Match {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Match")]
impl crate::UnityEngine::InputSystem::MatchResult_Match {
    pub fn get_control(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputControl,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputControl = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_control",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_device(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_device",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isOptional(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isOptional",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_requirement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_requirement",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_requirementIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_requirementIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputControlScheme_MatchResult {
    pub m_Result: crate::UnityEngine::InputSystem::MatchResult_Result,
    pub m_Score: f32,
    pub m_Devices: crate::UnityEngine::InputSystem::InputControlList_1<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    >,
    pub m_Controls: crate::UnityEngine::InputSystem::InputControlList_1<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
    pub m_Requirements: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputControlScheme_DeviceRequirement,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlScheme_MatchResult =>
    "UnityEngine.InputSystem"."InputControlScheme/MatchResult"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlScheme_MatchResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult")]
impl crate::UnityEngine::InputSystem::InputControlScheme_MatchResult {
    #[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Enumerator")]
    pub type Enumerator = crate::UnityEngine::InputSystem::MatchResult_Enumerator;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Match")]
    pub type Match = crate::UnityEngine::InputSystem::MatchResult_Match;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Result")]
    pub type Result = crate::UnityEngine::InputSystem::MatchResult_Result;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::UnityEngine::InputSystem::MatchResult_Match,
        >,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::UnityEngine::InputSystem::MatchResult_Match,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerable.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::MatchResult_Match,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::MatchResult_Match = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_devices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlList_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlList_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_devices", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasMissingOptionalDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hasMissingOptionalDevices",
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
    pub fn get_isSuccessfulMatch(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isSuccessfulMatch",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_score(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_score",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Result")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchResult_Result {
    AllSatisfied = 0i32,
    MissingOptional = 2i32,
    MissingRequired = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+MatchResult+Result")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::MatchResult_Result =>
    "UnityEngine.InputSystem"."InputControlScheme/MatchResult/Result"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+SchemeJson")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputControlScheme_SchemeJson {
    pub name: *mut crate::System::String,
    pub bindingGroup: *mut crate::System::String,
    pub devices: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::SchemeJson_DeviceJson,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+SchemeJson")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlScheme_SchemeJson =>
    "UnityEngine.InputSystem"."InputControlScheme/SchemeJson"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+SchemeJson")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlScheme_SchemeJson {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+SchemeJson")]
impl crate::UnityEngine::InputSystem::InputControlScheme_SchemeJson {
    #[cfg(feature = "UnityEngine+InputSystem+InputControlScheme+SchemeJson+DeviceJson")]
    pub type DeviceJson = crate::UnityEngine::InputSystem::SchemeJson_DeviceJson;
    pub fn ToScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToScheme",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
