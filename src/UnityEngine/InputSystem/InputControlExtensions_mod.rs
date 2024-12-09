#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputControlExtensions
    => "UnityEngine.InputSystem"."InputControlExtensions"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputControlExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputControlExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions")]
impl crate::UnityEngine::InputSystem::InputControlExtensions {
    #[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+ControlBuilder")]
    pub type ControlBuilder = crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
    pub type DeviceBuilder = crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder;
    #[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+Enumerate")]
    pub type Enumerate = crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
    )]
    pub type InputEventControlCollection = crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
    )]
    pub type InputEventControlEnumerator = crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputControlExtensions+_GetAllButtonPresses_d__43"
    )]
    pub type _GetAllButtonPresses_d__43 = crate::UnityEngine::InputSystem::InputControlExtensions__GetAllButtonPresses_d__43;
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputControlExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+ControlBuilder")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputControlExtensions_ControlBuilder {
    pub _control_k__BackingField: *mut crate::UnityEngine::InputSystem::InputControl,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+ControlBuilder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder =>
    "UnityEngine.InputSystem"."InputControlExtensions/ControlBuilder"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+ControlBuilder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+ControlBuilder")]
impl crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder {
    pub fn At(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "At",
            (device, index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn DontReset(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DontReset",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Finish",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsButton(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsButton",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsNoisy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNoisy",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsSynthetic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsSynthetic",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithAliases(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithAliases",
            (startIndex, count),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithChildren(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithChildren",
            (startIndex, count),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithDefaultState(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithDefaultState",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithDisplayName(
        &mut self,
        displayName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithDisplayName",
            (displayName),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithLayout(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithLayout",
            (layout),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithMinAndMax(
        &mut self,
        min: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        max: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithMinAndMax",
            (min, max),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithName(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithName",
            (name),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithParent(
        &mut self,
        parent: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithParent",
            (parent),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithProcessor<TProcessor, TValue>(
        &mut self,
        processor: TProcessor,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    >
    where
        TProcessor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithProcessor",
            (processor),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithShortDisplayName(
        &mut self,
        shortDisplayName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithShortDisplayName",
            (shortDisplayName),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithStateBlock(
        &mut self,
        stateBlock: crate::UnityEngine::InputSystem::LowLevel::InputStateBlock,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithStateBlock",
            (stateBlock),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithUsages(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithUsages",
            (startIndex, count),
        )?;
        Ok(__cordl_ret)
    }
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
    pub fn set_control(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_control",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputControlExtensions_DeviceBuilder {
    pub _device_k__BackingField: *mut crate::UnityEngine::InputSystem::InputDevice,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder =>
    "UnityEngine.InputSystem"."InputControlExtensions/DeviceBuilder"
);
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
impl crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder {
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Finish",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsNoisy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNoisy",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithChildren(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithChildren",
            (startIndex, count),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithControlAlias(
        &mut self,
        controlIndex: i32,
        alias: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithControlAlias",
            (controlIndex, alias),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithControlTree(
        &mut self,
        controlTreeNodes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        controlTreeIndicies: *mut quest_hook::libil2cpp::Il2CppArray<u16>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithControlTree",
            (controlTreeNodes, controlTreeIndicies),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithControlUsage(
        &mut self,
        controlIndex: i32,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithControlUsage",
            (controlIndex, usage, control),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithDisplayName(
        &mut self,
        displayName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithDisplayName",
            (displayName),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithLayout(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithLayout",
            (layout),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithName(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithName",
            (name),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithShortDisplayName(
        &mut self,
        shortDisplayName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithShortDisplayName",
            (shortDisplayName),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithStateBlock(
        &mut self,
        stateBlock: crate::UnityEngine::InputSystem::LowLevel::InputStateBlock,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithStateBlock",
            (stateBlock),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WithStateOffsetToControlIndexMap(
        &mut self,
        map: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithStateOffsetToControlIndexMap",
            (map),
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
    pub fn set_device(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_device",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+Enumerate")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputControlExtensions_Enumerate {
    IgnoreControlsInCurrentState = 2i32,
    IgnoreControlsInDefaultState = 1i32,
    IncludeNoisyControls = 8i32,
    IncludeNonLeafControls = 16i32,
    IncludeSyntheticControls = 4i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+Enumerate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlExtensions_Enumerate =>
    "UnityEngine.InputSystem"."InputControlExtensions/Enumerate"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputControlExtensions_InputEventControlCollection {
    pub m_Device: *mut crate::UnityEngine::InputSystem::InputDevice,
    pub m_EventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    pub m_Flags: crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate,
    pub m_MagnitudeThreshold: f32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection =>
    "UnityEngine.InputSystem"."InputControlExtensions/InputEventControlCollection"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
impl crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IEnumerable_UnityEngine_InputSystem_InputControl__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IEnumerable<UnityEngine.InputSystem.InputControl>.GetEnumerator",
            (),
        )?;
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
    pub fn get_eventPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_eventPtr",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputControlExtensions_InputEventControlEnumerator {
    pub m_Flags: crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate,
    pub m_Device: *mut crate::UnityEngine::InputSystem::InputDevice,
    pub m_StateOffsetToControlIndex: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub m_StateOffsetToControlIndexLength: i32,
    pub m_AllControls: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
    pub m_DefaultState: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_CurrentState: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_NoiseMask: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_EventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    pub m_CurrentControl: *mut crate::UnityEngine::InputSystem::InputControl,
    pub m_CurrentIndexInStateOffsetToControlIndexMap: i32,
    pub m_CurrentControlStateBitOffset: u32,
    pub m_EventState: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_CurrentBitOffset: u32,
    pub m_EndBitOffset: u32,
    pub m_MagnitudeThreshold: f32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator =>
    "UnityEngine.InputSystem"."InputControlExtensions/InputEventControlEnumerator"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
impl crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
    pub fn CheckCurrent(&mut self, numBits: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckCurrent",
            (numBits),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CheckDefault(&mut self, numBits: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckDefault",
            (numBits),
        )?;
        Ok(__cordl_ret)
    }
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
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        flags: crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate,
        magnitudeThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (eventPtr, device, flags, magnitudeThreshold),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputControl,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputControl = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
