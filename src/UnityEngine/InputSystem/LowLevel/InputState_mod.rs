#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+InputState")]
#[repr(C)]
#[derive(Debug)]
pub struct InputState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+InputState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputState";
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::LowLevel::InputState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::LowLevel::InputState {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
impl crate::UnityEngine::InputSystem::LowLevel::InputState {
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
    )]
    pub type StateChangeMonitorDelegate = crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate;
    pub fn AddChangeMonitorTimeout(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        monitor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        >,
        _cordl_time: f64,
        monitorIndex: i64,
        timerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
                            >,
                            f64,
                            i64,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("AddChangeMonitorTimeout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddChangeMonitorTimeout", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (control, monitor, _cordl_time, monitorIndex, timerIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddChangeMonitor_Action_4_i32_Action_4_1(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        valueChangeCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                f64,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                i64,
            >,
        >,
        monitorIndex: i32,
        timerExpiredCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                f64,
                i64,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_4<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::InputSystem::InputControl,
                                    >,
                                    f64,
                                    crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                                    i64,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_4<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::InputSystem::InputControl,
                                    >,
                                    f64,
                                    i64,
                                    i32,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
                        >,
                        4usize,
                    >("AddChangeMonitor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddChangeMonitor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (control, valueChangeCallback, monitorIndex, timerExpiredCallback),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddChangeMonitor_IInputStateChangeMonitor_i64_u32_0(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        monitor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        >,
        monitorIndex: i64,
        groupIndex: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
                            >,
                            i64,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("AddChangeMonitor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddChangeMonitor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (control, monitor, monitorIndex, groupIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Change_InputControl_ByRefMut_InputEventPtr2<TState>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        state: quest_hook::libil2cpp::ByRefMut<TState>,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TState>,
                            crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Change")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Change",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (control, state, updateType, eventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Change_InputControl_TState_InputEventPtr1<TState>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        state: TState,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            TState,
                            crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Change")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Change",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (control, state, updateType, eventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Change_InputDevice_InputEventPtr0(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputDevice,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Change")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Change",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (device, eventPtr, updateType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsIntegerFormat(
        format: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputSystem::Utilities::FourCC),
                        bool,
                        1usize,
                    >("IsIntegerFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsIntegerFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (format))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveChangeMonitor(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        monitor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        >,
        monitorIndex: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
                            >,
                            i64,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("RemoveChangeMonitor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveChangeMonitor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (control, monitor, monitorIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveChangeMonitorTimeout(
        monitor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        >,
        monitorIndex: i64,
        timerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
                            >,
                            i64,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("RemoveChangeMonitorTimeout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveChangeMonitorTimeout", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (monitor, monitorIndex, timerIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_onChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::InputSystem::InputDevice,
                                >,
                                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_onChange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_onChange", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentTime() -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), f64, 0usize>("get_currentTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_currentTime", 0usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentUpdateType() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
                        0usize,
                    >("get_currentUpdateType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_currentUpdateType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_updateCount() -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), u32, 0usize>("get_updateCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_updateCount", 0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::InputSystem::InputDevice,
                                >,
                                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_onChange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_onChange", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+InputState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InputState_StateChangeMonitorDelegate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub valueChangeCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            f64,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            i64,
        >,
    >,
    pub timerExpiredCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            f64,
            i64,
            i32,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputState/StateChangeMonitorDelegate";
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
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyControlStateChanged(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        _cordl_time: f64,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        monitorIndex: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            f64,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            i64,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("NotifyControlStateChanged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NotifyControlStateChanged", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (control, _cordl_time, eventPtr, monitorIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyTimerExpired(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        _cordl_time: f64,
        monitorIndex: i64,
        timerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            f64,
                            i64,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("NotifyTimerExpired")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NotifyTimerExpired", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (control, _cordl_time, monitorIndex, timerIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor>
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor>
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
        unsafe { std::mem::transmute(self) }
    }
}
