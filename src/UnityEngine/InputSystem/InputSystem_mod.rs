#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer+_data_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer+_data_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer =>
    "UnityEngine.InputSystem"."InputSystem/DeltaStateEventBuffer/<data>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer+_data_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer+_data_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer {}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct InputSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputSystem =>
    "UnityEngine.InputSystem"."InputSystem"
);
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
impl crate::UnityEngine::InputSystem::InputSystem {
    pub const kAssemblyVersion: &'static str = "1.7.0";
    pub const kDocUrl: &'static str = "https://docs.unity3d.com/Packages/com.unity.inputsystem@1.7";
    #[cfg(feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer")]
    pub type DeltaStateEventBuffer = crate::UnityEngine::InputSystem::InputSystem_DeltaStateEventBuffer;
    #[cfg(feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer")]
    pub type StateEventBuffer = crate::UnityEngine::InputSystem::InputSystem_StateEventBuffer;
    pub fn AddDeviceUsage_Il2CppString0(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        usage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDeviceUsage", (device, usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDeviceUsage_InternedString1(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDeviceUsage", (device, usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDevice_Il2CppString1<TDevice>(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<TDevice>
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDevice = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDevice", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDevice_Il2CppString_Il2CppString_Il2CppString0(
        layout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        variants: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDevice", (layout, name, variants))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDevice_InputDevice3(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDevice", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDevice_InputDeviceDescription2(
        description: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDevice", (description))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableAllEnabledActions() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisableAllEnabledActions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        keepSendingEvents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisableDevice", (device, keepSendingEvents))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableDevice", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitialized() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FindControl(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindControl", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindControls_ByRefMut2<TControl>(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        controls: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<TControl>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindControls", (path, controls))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindControls_Il2CppString0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlList_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlList_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindControls", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindControls_Il2CppString1<TControl>(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlList_1<TControl>,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlList_1<TControl> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindControls", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushDisconnectedDevices() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FlushDisconnectedDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDeviceById(
        deviceId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDeviceById", (deviceId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDevice_1<TDevice>() -> quest_hook::libil2cpp::Result<TDevice>
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDevice = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDevice", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDevice_Il2CppString0(
        nameOrLayout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDevice", (nameOrLayout))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDevice_Il2CppString4<TDevice>(
        usage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<TDevice>
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDevice = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDevice", (usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDevice_InternedString3<TDevice>(
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<TDevice>
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDevice = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDevice", (usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDevice_Type2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDevice", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNameOfBaseLayout(
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNameOfBaseLayout", (layoutName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsupportedDevices_0() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnsupportedDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsupportedDevices_List_1_1(
        descriptions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnsupportedDevices", (descriptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeInPlayer(
        runtime: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputRuntime,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeInPlayer", (runtime, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFirstLayoutBasedOnSecond(
        firstLayoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        secondLayoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsFirstLayoutBasedOnSecond", (firstLayoutName, secondLayoutName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ListEnabledActions_0() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::InputSystem::InputAction,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::InputSystem::InputAction,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ListEnabledActions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ListEnabledActions_List_1_1(
        actions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::InputSystem::InputAction,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ListEnabledActions", (actions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ListInteractions() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ListInteractions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ListLayouts() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ListLayouts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ListLayoutsBasedOn(
        baseLayout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ListLayoutsBasedOn", (baseLayout))?;
        Ok(__cordl_ret.into())
    }
    pub fn ListProcessors() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ListProcessors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLayout_1<TControl>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("LoadLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLayout_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("LoadLayout", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn PauseHaptics() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PauseHaptics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PerformDefaultPluginInitialization() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PerformDefaultPluginInitialization", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueConfigChangeEvent(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueConfigChangeEvent", (device, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueDeltaStateEvent<TDelta>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        delta: TDelta,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDelta: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueDeltaStateEvent", (control, delta, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueEvent_ByRefMut1<TEvent>(
        inputEvent: quest_hook::libil2cpp::ByRefMut<TEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEvent: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueEvent", (inputEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueEvent_InputEventPtr0(
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueEvent", (eventPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueStateEvent<TState>(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        state: TState,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueStateEvent", (device, state, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueTextEvent(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        character: char,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueTextEvent", (device, character, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterBindingComposite_Il2CppString1<T>(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterBindingComposite", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterBindingComposite_Type_Il2CppString0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterBindingComposite", (_cordl_type, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInteraction_Il2CppString1<T>(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterInteraction", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInteraction_Type_Il2CppString0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterInteraction", (_cordl_type, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterLayoutBuilder(
        buildMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
            >,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        baseLayout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        matches: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterLayoutBuilder", (buildMethod, name, baseLayout, matches))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterLayoutMatcher_Il2CppString_InputDeviceMatcher0(
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        matcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterLayoutMatcher", (layoutName, matcher))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterLayoutMatcher_InputDeviceMatcher1<TDevice>(
        matcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterLayoutMatcher", (matcher))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterLayoutOverride(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterLayoutOverride", (json, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterLayout_Il2CppString_Il2CppString_Nullable_1_2(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        matches: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterLayout", (json, name, matches))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterLayout_Il2CppString_Nullable_1_1<T>(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        matches: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterLayout", (name, matches))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterLayout_Type_Il2CppString_Nullable_1_0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        matches: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterLayout", (_cordl_type, name, matches))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterPrecompiledLayout<TDevice>(
        metadata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterPrecompiledLayout", (metadata))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProcessor_Il2CppString1<T>(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterProcessor", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProcessor_Type_Il2CppString0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterProcessor", (_cordl_type, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveDevice", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDeviceUsage_Il2CppString0(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        usage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveDeviceUsage", (device, usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDeviceUsage_InternedString1(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveDeviceUsage", (device, usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveLayout(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveLayout", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        alsoResetDontResetControls: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetDevice", (device, alsoResetDontResetControls))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetHaptics() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetHaptics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResumeHaptics() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResumeHaptics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RunInitialUpdate() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunInitialUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RunInitializeInPlayer() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunInitializeInPlayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDeviceUsage_Il2CppString0(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        usage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDeviceUsage", (device, usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDeviceUsage_InternedString1(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDeviceUsage", (device, usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindMatchingLayout(
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFindMatchingLayout", (deviceDescription))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetBindingComposite(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetBindingComposite", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInteraction(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetInteraction", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetProcessor(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetProcessor", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryResetDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryResetDevice", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySyncDevice(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrySyncDevice", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update_InputUpdateType1(
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Update", (updateType))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onActionChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                crate::UnityEngine::InputSystem::InputActionChange,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onActionChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onAfterUpdate(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onAfterUpdate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onBeforeUpdate(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onBeforeUpdate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onDeviceChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::InputSystem::InputDevice,
                crate::UnityEngine::InputSystem::InputDeviceChange,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onDeviceChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onDeviceCommand(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputDeviceCommandDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onDeviceCommand", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onFindLayoutForDevice(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceFindControlLayoutDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onFindLayoutForDevice", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onLayoutChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::UnityEngine::InputSystem::InputControlLayoutChange,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onLayoutChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onSettingsChange(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onSettingsChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_devices() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_devices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disconnectedDevices() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_disconnectedDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isProcessingEvents() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isProcessingEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_metrics() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputMetrics,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputMetrics = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_metrics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onAnyButtonPress() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<
                *mut crate::UnityEngine::InputSystem::InputControl,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IObservable_1<
                *mut crate::UnityEngine::InputSystem::InputControl,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_onAnyButtonPress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onEvent() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventListener = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_onEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pollingFrequency() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pollingFrequency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_remoting() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputRemoting>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputRemoting,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_remoting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_runInBackground() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_runInBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_settings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputSettings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputSettings,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_settings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_version", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onActionChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                crate::UnityEngine::InputSystem::InputActionChange,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onActionChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onAfterUpdate(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onAfterUpdate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onBeforeUpdate(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onBeforeUpdate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onDeviceChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::InputSystem::InputDevice,
                crate::UnityEngine::InputSystem::InputDeviceChange,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onDeviceChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onDeviceCommand(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputDeviceCommandDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onDeviceCommand", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onFindLayoutForDevice(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceFindControlLayoutDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onFindLayoutForDevice", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onLayoutChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::UnityEngine::InputSystem::InputControlLayoutChange,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onLayoutChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onSettingsChange(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onSettingsChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_onEvent(
        value: crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_onEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pollingFrequency(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_pollingFrequency", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_runInBackground(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_runInBackground", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_settings(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_settings", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InputSystem::InputSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputSystem_DeltaStateEventBuffer {
    pub stateEvent: crate::UnityEngine::InputSystem::LowLevel::DeltaStateEvent,
    pub data: crate::UnityEngine::InputSystem::DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer,
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputSystem_DeltaStateEventBuffer =>
    "UnityEngine.InputSystem"."InputSystem/DeltaStateEventBuffer"
);
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputSystem_DeltaStateEventBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer")]
impl crate::UnityEngine::InputSystem::InputSystem_DeltaStateEventBuffer {
    pub const kMaxSize: i32 = 512i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer+_data_e__FixedBuffer"
    )]
    pub type _data_e__FixedBuffer = crate::UnityEngine::InputSystem::DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer;
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputSystem_StateEventBuffer {
    pub stateEvent: crate::UnityEngine::InputSystem::LowLevel::StateEvent,
    pub data: crate::UnityEngine::InputSystem::StateEventBuffer_InputSystem__data_e__FixedBuffer,
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputSystem_StateEventBuffer => "UnityEngine.InputSystem"
    ."InputSystem/StateEventBuffer"
);
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputSystem_StateEventBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer")]
impl crate::UnityEngine::InputSystem::InputSystem_StateEventBuffer {
    pub const kMaxSize: i32 = 512i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer+_data_e__FixedBuffer"
    )]
    pub type _data_e__FixedBuffer = crate::UnityEngine::InputSystem::StateEventBuffer_InputSystem__data_e__FixedBuffer;
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer+_data_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StateEventBuffer_InputSystem__data_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer+_data_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::StateEventBuffer_InputSystem__data_e__FixedBuffer =>
    "UnityEngine.InputSystem"."InputSystem/StateEventBuffer/<data>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer+_data_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::StateEventBuffer_InputSystem__data_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer+_data_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::StateEventBuffer_InputSystem__data_e__FixedBuffer {}
