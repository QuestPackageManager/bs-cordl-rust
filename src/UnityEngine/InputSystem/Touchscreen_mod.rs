#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
#[repr(C)]
#[derive(Debug)]
pub struct Touchscreen {
    __cordl_parent: crate::UnityEngine::InputSystem::Pointer,
    pub _primaryTouch_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::TouchControl,
    >,
    pub _touches_k__BackingField: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Touchscreen {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "Touchscreen";
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
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Touchscreen {
    type Target = crate::UnityEngine::InputSystem::Pointer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Touchscreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
impl crate::UnityEngine::InputSystem::Touchscreen {
    pub fn FinishSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("FinishSetup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FinishSetup", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn MakeCurrent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("MakeCurrent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MakeCurrent", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn MergeForward(
        currentEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        nextEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                    crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                ),
                bool,
                2usize,
            >("MergeForward")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MergeForward", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (currentEventPtr, nextEventPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnNextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnNextUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnNextUpdate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnRemoved(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnRemoved")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnRemoved", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnStateEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::LowLevel::InputEventPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnStateEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnStateEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TriggerTap(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
        state: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::TouchState,
        >,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Controls::TouchControl,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::LowLevel::TouchState,
                    >,
                    crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TriggerTap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TriggerTap", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (control, state, eventPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_ICustomDeviceReset_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UnityEngine.InputSystem.LowLevel.ICustomDeviceReset.Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnityEngine.InputSystem.LowLevel.ICustomDeviceReset.Reset",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IEventMerger_MergeForward(
        &mut self,
        currentEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        nextEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                    crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                ),
                bool,
                2usize,
            >("UnityEngine.InputSystem.LowLevel.IEventMerger.MergeForward")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnityEngine.InputSystem.LowLevel.IEventMerger.MergeForward",
                    2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (currentEventPtr, nextEventPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateCallbackReceiver_GetStateOffsetForEvent(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        offset: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                    crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                    quest_hook::libil2cpp::ByRefMut<u32>,
                ),
                bool,
                3usize,
            >(
                "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.GetStateOffsetForEvent",
            )
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.GetStateOffsetForEvent",
                    3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (control, eventPtr, offset))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateCallbackReceiver_OnNextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >(
                "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.OnNextUpdate",
            )
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.OnNextUpdate",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateCallbackReceiver_OnStateEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::LowLevel::InputEventPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >(
                "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.OnStateEvent",
            )
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self,
                    "UnityEngine.InputSystem.LowLevel.IInputStateCallbackReceiver.OnStateEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_current() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
                0usize,
            >("get_current")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_current", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Touchscreen,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_primaryTouch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::TouchControl,
                >,
                0usize,
            >("get_primaryTouch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_primaryTouch", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_touchControlArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::TouchControl,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                    >,
                >,
                0usize,
            >("get_touchControlArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_touchControlArray", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::TouchControl,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_touches(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::Controls::TouchControl,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Controls::TouchControl,
                    >,
                >,
                0usize,
            >("get_touches")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_touches", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::Controls::TouchControl,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_current(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Touchscreen,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_current")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_current", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_primaryTouch(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::TouchControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::TouchControl,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_primaryTouch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_primaryTouch", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_touchControlArray(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Controls::TouchControl,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::Controls::TouchControl,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_touchControlArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_touchControlArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_touches(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::Controls::TouchControl,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Controls::TouchControl,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_touches")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_touches", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InputSystem::Touchscreen {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::ICustomDeviceReset>
for crate::UnityEngine::InputSystem::Touchscreen {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::ICustomDeviceReset {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::ICustomDeviceReset>
for crate::UnityEngine::InputSystem::Touchscreen {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::ICustomDeviceReset {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IEventMerger>
for crate::UnityEngine::InputSystem::Touchscreen {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IEventMerger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IEventMerger>
for crate::UnityEngine::InputSystem::Touchscreen {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IEventMerger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver>
for crate::UnityEngine::InputSystem::Touchscreen {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Touchscreen")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver>
for crate::UnityEngine::InputSystem::Touchscreen {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
