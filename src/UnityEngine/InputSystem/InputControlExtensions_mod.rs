#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputControlExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputControlExtensions";
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputControlExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputControlExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
    pub fn AccumulateValueInEvent_InputControl_1_Il2CppObject_InputEventPtr0(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<f32>,
        >,
        currentStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        newState: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<f32>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AccumulateValueInEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AccumulateValueInEvent", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, currentStatePtr, newState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AccumulateValueInEvent_InputControl_1_Il2CppObject_InputEventPtr1(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<crate::UnityEngine::Vector2>,
        >,
        currentStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        newState: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<
                                    crate::UnityEngine::Vector2,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AccumulateValueInEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AccumulateValueInEvent", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, currentStatePtr, newState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildPath(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        deviceLayout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        builder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        3usize,
                    >("BuildPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "BuildPath", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (control, deviceLayout, builder))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckStateIsAtDefaultIgnoringNoise_Il2CppObject1(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        2usize,
                    >("CheckStateIsAtDefaultIgnoringNoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckStateIsAtDefaultIgnoringNoise", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, statePtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckStateIsAtDefaultIgnoringNoise_InputControl0(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        bool,
                        1usize,
                    >("CheckStateIsAtDefaultIgnoringNoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckStateIsAtDefaultIgnoringNoise", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (control))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckStateIsAtDefault_Il2CppObject_Il2CppObject1(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        maskPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        3usize,
                    >("CheckStateIsAtDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckStateIsAtDefault", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, statePtr, maskPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckStateIsAtDefault_InputControl0(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        bool,
                        1usize,
                    >("CheckStateIsAtDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckStateIsAtDefault", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (control))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareStateIgnoringNoise(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        2usize,
                    >("CompareStateIgnoringNoise")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CompareStateIgnoringNoise", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, statePtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompareState_Il2CppObject0(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        firstStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secondStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        maskPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        4usize,
                    >("CompareState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CompareState", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked((), (control, firstStatePtr, secondStatePtr, maskPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompareState_InputControl_Il2CppObject_Il2CppObject1(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        maskPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        3usize,
                    >("CompareState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CompareState", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, statePtr, maskPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyState_ByRefMut1<TState>(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        state: quest_hook::libil2cpp::ByRefMut<TState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputDevice,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TState>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CopyState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CopyState", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (device, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyState_Il2CppObject_i32_0(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSizeInBytes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputDevice,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CopyState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CopyState", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (device, buffer, bufferSizeInBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnumerateChangedControls(
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        magnitudeThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputDevice,
                            >,
                            f32,
                        ),
                        crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection,
                        3usize,
                    >("EnumerateChangedControls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnumerateChangedControls", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection = unsafe {
            method.invoke_unchecked((), (eventPtr, device, magnitudeThreshold))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnumerateControls(
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        flags: crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        magnitudeThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputDevice,
                            >,
                            f32,
                        ),
                        crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection,
                        4usize,
                    >("EnumerateControls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EnumerateControls", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection = unsafe {
            method.invoke_unchecked((), (eventPtr, flags, device, magnitudeThreshold))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindControlsRecursive<TControl>(
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        controls: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<TControl>,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TControl, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<TControl>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<TControl, bool>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("FindControlsRecursive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FindControlsRecursive", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (parent, controls, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindInParentChain<TControl>(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<TControl>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        TControl,
                        1usize,
                    >("FindInParentChain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FindInParentChain", 1usize
                        )
                    })
            });
        let __cordl_ret: TControl = unsafe { method.invoke_unchecked((), (control))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllButtonPresses(
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        magnitude: f32,
        buttonControlsOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            f32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::InputSystem::InputControl,
                                >,
                            >,
                        >,
                        3usize,
                    >("GetAllButtonPresses")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetAllButtonPresses", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            >,
        > = unsafe {
            method.invoke_unchecked((), (eventPtr, magnitude, buttonControlsOnly))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstButtonPressOrNull(
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        magnitude: f32,
        buttonControlsOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            f32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >,
                        3usize,
                    >("GetFirstButtonPressOrNull")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetFirstButtonPressOrNull", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = unsafe {
            method.invoke_unchecked((), (eventPtr, magnitude, buttonControlsOnly))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStatePtrFromStateEvent(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        2usize,
                    >("GetStatePtrFromStateEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetStatePtrFromStateEvent", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (control, eventPtr))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStatePtrFromStateEventUnchecked(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        eventType: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            crate::UnityEngine::InputSystem::Utilities::FourCC,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("GetStatePtrFromStateEventUnchecked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetStatePtrFromStateEventUnchecked", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (control, eventPtr, eventType))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasButtonPress(
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        magnitude: f32,
        buttonControlsOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            f32,
                            bool,
                        ),
                        bool,
                        3usize,
                    >("HasButtonPress")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HasButtonPress", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (eventPtr, magnitude, buttonControlsOnly))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasValueChangeInEvent(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        bool,
                        2usize,
                    >("HasValueChangeInEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HasValueChangeInEvent", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, eventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasValueChangeInState(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        2usize,
                    >("HasValueChangeInState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "HasValueChangeInState", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, statePtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsActuated(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        threshold: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            f32,
                        ),
                        bool,
                        2usize,
                    >("IsActuated")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsActuated", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, threshold))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsPressed(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        buttonPressPoint: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            f32,
                        ),
                        bool,
                        2usize,
                    >("IsPressed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsPressed", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, buttonPressPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn QueueValueChange<TValue>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
        value: TValue,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<TValue>,
                            >,
                            TValue,
                            f64,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("QueueValueChange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "QueueValueChange", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, value, _cordl_time))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadDefaultValueAsObject(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("ReadDefaultValueAsObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadDefaultValueAsObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (control))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnprocessedValueFromEvent_ByRefMut1<TValue>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
        inputEvent: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<TValue>,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            quest_hook::libil2cpp::ByRefMut<TValue>,
                        ),
                        bool,
                        3usize,
                    >("ReadUnprocessedValueFromEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadUnprocessedValueFromEvent", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, inputEvent, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnprocessedValueFromEvent_InputControl_1_InputEventPtr0<TValue>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<TValue>,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        TValue,
                        2usize,
                    >("ReadUnprocessedValueFromEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadUnprocessedValueFromEvent", 2usize
                        )
                    })
            });
        let __cordl_ret: TValue = unsafe {
            method.invoke_unchecked((), (control, eventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueAsObject(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("ReadValueAsObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadValueAsObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (control))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueFromEventAsObject(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        inputEvent: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        2usize,
                    >("ReadValueFromEventAsObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadValueFromEventAsObject", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (control, inputEvent))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueFromEvent_ByRefMut1<TValue>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
        inputEvent: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<TValue>,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            quest_hook::libil2cpp::ByRefMut<TValue>,
                        ),
                        bool,
                        3usize,
                    >("ReadValueFromEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadValueFromEvent", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, inputEvent, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueFromEvent_InputControl_1_InputEventPtr0<TValue>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
        inputEvent: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<TValue>,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        TValue,
                        2usize,
                    >("ReadValueFromEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadValueFromEvent", 2usize
                        )
                    })
            });
        let __cordl_ret: TValue = unsafe {
            method.invoke_unchecked((), (control, inputEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueIntoBuffer(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ReadValueIntoBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadValueIntoBuffer", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, buffer, bufferSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetToDefaultStateInEvent(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        bool,
                        2usize,
                    >("ResetToDefaultStateInEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ResetToDefaultStateInEvent", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (control, eventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup_InputControl0(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Setup", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked((), (control))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup_InputDevice_i32_i32_i32_1(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        controlCount: i32,
        usageCount: i32,
        aliasCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputDevice,
                            >,
                            i32,
                            i32,
                            i32,
                        ),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        4usize,
                    >("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Setup", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked((), (device, controlCount, usageCount, aliasCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueFromObjectIntoEvent(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteValueFromObjectIntoEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteValueFromObjectIntoEvent", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, eventPtr, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueIntoEvent_InputControl0<TValue>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        value: TValue,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            TValue,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteValueIntoEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteValueIntoEvent", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, value, eventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueIntoEvent_InputControl_1_1<TValue>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
        value: TValue,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<TValue>,
                            >,
                            TValue,
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteValueIntoEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteValueIntoEvent", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, value, eventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueIntoState_InputControl_1_Il2CppObject3<TValue>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<TValue>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteValueIntoState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteValueIntoState", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, statePtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueIntoState_InputControl_1_TValue_ByRefMut4<TValue, TState>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
        value: TValue,
        state: quest_hook::libil2cpp::ByRefMut<TState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<TValue>,
                            >,
                            TValue,
                            quest_hook::libil2cpp::ByRefMut<TState>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteValueIntoState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteValueIntoState", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, value, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueIntoState_InputControl_1_TValue_Il2CppObject2<TValue>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
        value: TValue,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl_1<TValue>,
                            >,
                            TValue,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteValueIntoState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteValueIntoState", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, value, statePtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueIntoState_InputControl_Il2CppObject0(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteValueIntoState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteValueIntoState", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, statePtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueIntoState_InputControl_TValue_Il2CppObject1<TValue>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        value: TValue,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                            TValue,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteValueIntoState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteValueIntoState", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, value, statePtr))?
        };
        Ok(__cordl_ret.into())
    }
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputControlExtensions_ControlBuilder {
    pub _control_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputControl,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+ControlBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputControlExtensions/ControlBuilder";
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+ControlBuilder")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+ControlBuilder")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+ControlBuilder")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+ControlBuilder")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder {
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
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputDevice,
                            >,
                            i32,
                        ),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        2usize,
                    >("At")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "At", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (device, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DontReset(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("DontReset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "DontReset", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Finish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Finish", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsButton(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("IsButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsButton", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsNoisy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("IsNoisy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsNoisy", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSynthetic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("IsSynthetic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsSynthetic", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithAliases(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        2usize,
                    >("WithAliases")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithAliases", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (startIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithChildren(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        2usize,
                    >("WithChildren")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithChildren", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (startIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithDefaultState(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::PrimitiveValue),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("WithDefaultState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithDefaultState", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithDisplayName(
        &mut self,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("WithDisplayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithDisplayName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (displayName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithLayout(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("WithLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (layout))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithMinAndMax(
        &mut self,
        min: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
        max: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
                            crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
                        ),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        2usize,
                    >("WithMinAndMax")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithMinAndMax", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (min, max))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("WithName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithParent(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("WithParent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithParent", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (parent))?
        };
        Ok(__cordl_ret.into())
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (TProcessor),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("WithProcessor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithProcessor", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (processor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithShortDisplayName(
        &mut self,
        shortDisplayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("WithShortDisplayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithShortDisplayName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (shortDisplayName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithStateBlock(
        &mut self,
        stateBlock: crate::UnityEngine::InputSystem::LowLevel::InputStateBlock,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::LowLevel::InputStateBlock),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        1usize,
                    >("WithStateBlock")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithStateBlock", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (stateBlock))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithUsages(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32),
                        crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder,
                        2usize,
                    >("WithUsages")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithUsages", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_ControlBuilder = unsafe {
            method.invoke_unchecked(self, (startIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_control(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >,
                        0usize,
                    >("get_control")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_control", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_control(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_control")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_control", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputControlExtensions_DeviceBuilder {
    pub _device_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputDevice,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputControlExtensions/DeviceBuilder";
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+DeviceBuilder")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Finish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Finish", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsNoisy(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        1usize,
                    >("IsNoisy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsNoisy", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithChildren(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        2usize,
                    >("WithChildren")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithChildren", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (startIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithControlAlias(
        &mut self,
        controlIndex: i32,
        alias: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                        ),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        2usize,
                    >("WithControlAlias")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithControlAlias", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (controlIndex, alias))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithControlTree(
        &mut self,
        controlTreeNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        controlTreeIndicies: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u16>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u16>,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        2usize,
                    >("WithControlTree")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithControlTree", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (controlTreeNodes, controlTreeIndicies))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithControlUsage(
        &mut self,
        controlIndex: i32,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            crate::UnityEngine::InputSystem::Utilities::InternedString,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        ),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        3usize,
                    >("WithControlUsage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithControlUsage", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (controlIndex, usage, control))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithDisplayName(
        &mut self,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        1usize,
                    >("WithDisplayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithDisplayName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (displayName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithLayout(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::Utilities::InternedString),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        1usize,
                    >("WithLayout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithLayout", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (layout))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        1usize,
                    >("WithName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithShortDisplayName(
        &mut self,
        shortDisplayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        1usize,
                    >("WithShortDisplayName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithShortDisplayName", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (shortDisplayName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithStateBlock(
        &mut self,
        stateBlock: crate::UnityEngine::InputSystem::LowLevel::InputStateBlock,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::LowLevel::InputStateBlock),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        1usize,
                    >("WithStateBlock")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithStateBlock", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (stateBlock))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WithStateOffsetToControlIndexMap(
        &mut self,
        map: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u32>,
                        >),
                        crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder,
                        1usize,
                    >("WithStateOffsetToControlIndexMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WithStateOffsetToControlIndexMap", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_DeviceBuilder = unsafe {
            method.invoke_unchecked(self, (map))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_device(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputDevice,
                        >,
                        0usize,
                    >("get_device")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_device", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_device(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputDevice,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_device")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_device", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+Enumerate")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputControlExtensions_Enumerate {
    #[default]
    IgnoreControlsInCurrentState = 2i32,
    IgnoreControlsInDefaultState = 1i32,
    IncludeNoisyControls = 8i32,
    IncludeNonLeafControls = 16i32,
    IncludeSyntheticControls = 4i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+Enumerate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputControlExtensions/Enumerate";
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+Enumerate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+Enumerate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+Enumerate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate {
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
#[cfg(feature = "UnityEngine+InputSystem+InputControlExtensions+Enumerate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate {
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
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputControlExtensions_InputEventControlCollection {
    pub m_Device: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputDevice,
    >,
    pub m_EventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    pub m_Flags: crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate,
    pub m_MagnitudeThreshold: f32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputControlExtensions/InputEventControlCollection";
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
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
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
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
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
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator,
                        0usize,
                    >("GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetEnumerator", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IEnumerable_UnityEngine_InputSystem_InputControl__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerator_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::InputSystem::InputControl,
                                >,
                            >,
                        >,
                        0usize,
                    >(
                        "System.Collections.Generic.IEnumerable<UnityEngine.InputSystem.InputControl>.GetEnumerator",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "System.Collections.Generic.IEnumerable<UnityEngine.InputSystem.InputControl>.GetEnumerator",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        0usize,
                    >("System.Collections.IEnumerable.GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "System.Collections.IEnumerable.GetEnumerator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_eventPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                        0usize,
                    >("get_eventPtr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_eventPtr", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    >,
>
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    >,
>
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlCollection"
)]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlCollection {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputControlExtensions_InputEventControlEnumerator {
    pub m_Flags: crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate,
    pub m_Device: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputDevice,
    >,
    pub m_StateOffsetToControlIndex: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u32>,
    >,
    pub m_StateOffsetToControlIndexLength: i32,
    pub m_AllControls: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        >,
    >,
    pub m_DefaultState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_CurrentState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_NoiseMask: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_EventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    pub m_CurrentControl: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputControl,
    >,
    pub m_CurrentIndexInStateOffsetToControlIndexMap: i32,
    pub m_CurrentControlStateBitOffset: u32,
    pub m_EventState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_CurrentBitOffset: u32,
    pub m_EndBitOffset: u32,
    pub m_MagnitudeThreshold: f32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputControlExtensions/InputEventControlEnumerator";
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
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
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
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
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
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(u32), bool, 1usize>("CheckCurrent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckCurrent", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (numBits))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckDefault(&mut self, numBits: u32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(u32), bool, 1usize>("CheckDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckDefault", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (numBits))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Dispose", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Reset", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("System.Collections.IEnumerator.get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "System.Collections.IEnumerator.get_Current",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        flags: crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate,
        magnitudeThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputDevice,
                            >,
                            crate::UnityEngine::InputSystem::InputControlExtensions_Enumerate,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventPtr, device, flags, magnitudeThreshold))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >,
                        0usize,
                    >("get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Current", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
impl AsRef<
    crate::System::Collections::Generic::IEnumerator_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    >,
>
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerator_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
impl AsMut<
    crate::System::Collections::Generic::IEnumerator_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    >,
>
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputControlExtensions+InputEventControlEnumerator"
)]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputControlExtensions_InputEventControlEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
