#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BindingState_InputActionState_Flags {
    #[default]
    ChainsWithNext = 1i32,
    Composite = 4i32,
    EndOfChain = 2i32,
    InitialStateCheckPending = 16i32,
    PartOfComposite = 8i32,
    WantsInitialStateCheck = 32i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState+Flags")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionState/BindingState/Flags";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState+Flags")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState+Flags")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState+Flags")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState+Flags")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub maps: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
        >,
    >,
    pub controls: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        >,
    >,
    pub interactions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<Blacklisted>,
    >,
    pub processors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputProcessor>,
        >,
    >,
    pub composites: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::InputBindingComposite,
            >,
        >,
    >,
    pub totalProcessorCount: i32,
    pub memory: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
    pub m_OnBeforeUpdateHooked: bool,
    pub m_OnAfterUpdateHooked: bool,
    pub m_InProcessControlStateChange: bool,
    pub m_CurrentlyProcessingThisEvent: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    pub m_OnBeforeUpdateDelegate: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_OnAfterUpdateDelegate: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionState";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputActionState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputActionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl crate::UnityEngine::InputSystem::InputActionState {
    pub const kInvalidIndex: i32 = -1i32;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
    pub type ActionMapIndices = crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
    pub type BindingState = crate::UnityEngine::InputSystem::InputActionState_BindingState;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
    pub type GlobalState = crate::UnityEngine::InputSystem::InputActionState_GlobalState;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
    pub type InteractionState = crate::UnityEngine::InputSystem::InputActionState_InteractionState;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
    pub type TriggerState = crate::UnityEngine::InputSystem::InputActionState_TriggerState;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
    pub type UnmanagedMemory = crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory;
    pub fn AddToGlobalList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("AddToGlobalList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "AddToGlobalList", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyProcessors<TValue>(
        &mut self,
        bindingIndex: i32,
        value: TValue,
        controlOfType: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    TValue,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl_1<TValue>,
                    >,
                ),
                TValue,
                3usize,
            >("ApplyProcessors")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ApplyProcessors", 3usize
                )
            });
        let __cordl_ret: TValue = unsafe {
            method.invoke_unchecked(self, (bindingIndex, value, controlOfType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CallActionListeners(
        &mut self,
        actionIndex: i32,
        actionMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        >,
        phase: crate::UnityEngine::InputSystem::InputActionPhase,
        listeners: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
                    >,
                >,
            >,
        >,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputActionMap,
                    >,
                    crate::UnityEngine::InputSystem::InputActionPhase,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    crate::UnityEngine::InputSystem::InputAction_CallbackContext,
                                >,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("CallActionListeners")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "CallActionListeners",
                    5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (actionIndex, actionMap, phase, listeners, callbackName),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CanUseDevice(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputDevice,
                >),
                bool,
                1usize,
            >("CanUseDevice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "CanUseDevice", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (device))? };
        Ok(__cordl_ret.into())
    }
    pub fn ChangePhaseOfAction(
        &mut self,
        newPhase: crate::UnityEngine::InputSystem::InputActionPhase,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        phaseAfterPerformedOrCanceled: crate::UnityEngine::InputSystem::InputActionPhase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::InputActionPhase,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                    >,
                    crate::UnityEngine::InputSystem::InputActionPhase,
                ),
                bool,
                3usize,
            >("ChangePhaseOfAction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ChangePhaseOfAction",
                    3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (newPhase, trigger, phaseAfterPerformedOrCanceled),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangePhaseOfActionInternal(
        &mut self,
        actionIndex: i32,
        actionState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        newPhase: crate::UnityEngine::InputSystem::InputActionPhase,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::UnityEngine::InputSystem::InputActionPhase,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ChangePhaseOfActionInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ChangePhaseOfActionInternal", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (actionIndex, actionState, newPhase, trigger))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangePhaseOfInteraction(
        &mut self,
        newPhase: crate::UnityEngine::InputSystem::InputActionPhase,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        phaseAfterPerformed: crate::UnityEngine::InputSystem::InputActionPhase,
        processNextInteractionOnCancel: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::InputActionPhase,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                    >,
                    crate::UnityEngine::InputSystem::InputActionPhase,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ChangePhaseOfInteraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ChangePhaseOfInteraction",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        newPhase,
                        trigger,
                        phaseAfterPerformed,
                        processNextInteractionOnCancel,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClaimDataFrom(
        &mut self,
        resolver: crate::UnityEngine::InputSystem::InputBindingResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::InputBindingResolver),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ClaimDataFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ClaimDataFrom", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (resolver))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionState>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionState,
                >,
                0usize,
            >("Clone")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "Clone", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionState,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CompactGlobalList() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("CompactGlobalList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "CompactGlobalList", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeControlGroupingIfNecessary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ComputeControlGroupingIfNecessary")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ComputeControlGroupingIfNecessary", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeferredResolutionOfBindings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("DeferredResolutionOfBindings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "DeferredResolutionOfBindings", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Destroy(
        &mut self,
        isFinalizing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Destroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "Destroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (isFinalizing))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyAllActionMapStates() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("DestroyAllActionMapStates")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "DestroyAllActionMapStates",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableAllActions_1() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("DisableAllActions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "DisableAllActions", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableAllActions_InputActionMap0(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionMap,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DisableAllActions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "DisableAllActions", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (map))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableControls_InputAction1(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputAction,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DisableControls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "DisableControls", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableControls_InputActionMap0(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionMap,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DisableControls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "DisableControls", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (map))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableControls_i32_i32_i32_2(
        &mut self,
        mapIndex: i32,
        controlStartIndex: i32,
        numControls: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DisableControls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "DisableControls", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (mapIndex, controlStartIndex, numControls))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableSingleAction(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputAction,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DisableSingleAction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "DisableSingleAction",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableAllActions(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionMap,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EnableAllActions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "EnableAllActions", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (map))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableControls_InputAction1(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputAction,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EnableControls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "EnableControls", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableControls_InputActionMap0(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionMap,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EnableControls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "EnableControls", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (map))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableControls_i32_i32_i32_2(
        &mut self,
        mapIndex: i32,
        controlStartIndex: i32,
        numControls: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("EnableControls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "EnableControls", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (mapIndex, controlStartIndex, numControls))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableSingleAction(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputAction,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EnableSingleAction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "EnableSingleAction", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateCompositePartMagnitude(
        &mut self,
        bindingIndex: i32,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), f32, 2usize>("EvaluateCompositePartMagnitude")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "EvaluateCompositePartMagnitude", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked(self, (bindingIndex, partNumber))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FetchActionState(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputAction,
                >),
                quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                >,
                1usize,
            >("FetchActionState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "FetchActionState", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        > = unsafe { method.invoke_unchecked(self, (action))? };
        Ok(__cordl_ret.into())
    }
    pub fn FetchMapIndices(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionMap,
                >),
                crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices,
                1usize,
            >("FetchMapIndices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "FetchMapIndices", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices = unsafe {
            method.invoke_unchecked(self, (map))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Finalize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "Finalize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindAllEnabledActions(
        result: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputAction,
                        >,
                    >,
                >),
                i32,
                1usize,
            >("FindAllEnabledActions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "FindAllEnabledActions",
                    1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn FindControlIndexOnBinding(
        &mut self,
        bindingIndex: i32,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputControl,
                    >,
                ),
                i32,
                2usize,
            >("FindControlIndexOnBinding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "FindControlIndexOnBinding",
                    2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (bindingIndex, control))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FinishBindingCompositeSetups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("FinishBindingCompositeSetups")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "FinishBindingCompositeSetups", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FinishBindingResolution(
        &mut self,
        hasEnabledActions: bool,
        oldMemory: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
        activeControls: crate::UnityEngine::InputSystem::InputControlList_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        >,
        isFullResolve: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
                    crate::UnityEngine::InputSystem::InputControlList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("FinishBindingResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "FinishBindingResolution",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (hasEnabledActions, oldMemory, activeControls, isFullResolve),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionBindingStartIndexAndCount(
        &mut self,
        actionIndex: i32,
        bindingCount: quest_hook::libil2cpp::ByRefMut<u16>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, quest_hook::libil2cpp::ByRefMut<u16>),
                u16,
                2usize,
            >("GetActionBindingStartIndexAndCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetActionBindingStartIndexAndCount", 2usize
                )
            });
        let __cordl_ret: u16 = unsafe {
            method.invoke_unchecked(self, (actionIndex, bindingCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionMap(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionMap,
                >,
                1usize,
            >("GetActionMap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetActionMap", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        > = unsafe { method.invoke_unchecked(self, (bindingIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionOrNoneString(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("GetActionOrNoneString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetActionOrNoneString",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (trigger))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionOrNull_ByRefMut1(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                >),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
                1usize,
            >("GetActionOrNull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetActionOrNull", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = unsafe { method.invoke_unchecked(self, (trigger))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionOrNull_i32_0(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
                1usize,
            >("GetActionOrNull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetActionOrNull", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = unsafe { method.invoke_unchecked(self, (bindingIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBinding(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::InputSystem::InputBinding>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::InputSystem::InputBinding,
                >,
                1usize,
            >("GetBinding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetBinding", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBinding,
        > = unsafe { method.invoke_unchecked(self, (bindingIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingIndexInMap(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetBindingIndexInMap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetBindingIndexInMap",
                    1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (bindingIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingIndexInState(
        &mut self,
        mapIndex: i32,
        bindingIndexInMap: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("GetBindingIndexInState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetBindingIndexInState",
                    2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (mapIndex, bindingIndexInMap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingState(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_BindingState,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::InputSystem::InputActionState_BindingState,
                >,
                1usize,
            >("GetBindingState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetBindingState", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_BindingState,
        > = unsafe { method.invoke_unchecked(self, (bindingIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetComplexityFromMonitorIndex(
        mapControlAndBindingIndex: i64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), i32, 1usize>("GetComplexityFromMonitorIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetComplexityFromMonitorIndex", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (mapControlAndBindingIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCompositePartPressTime(
        &mut self,
        bindingIndex: i32,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), f64, 2usize>("GetCompositePartPressTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetCompositePartPressTime",
                    2usize
                )
            });
        let __cordl_ret: f64 = unsafe {
            method.invoke_unchecked(self, (bindingIndex, partNumber))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetControl(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                >),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                1usize,
            >("GetControl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetControl", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = unsafe { method.invoke_unchecked(self, (trigger))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetInteractionOrNull(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<Blacklisted> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                >),
                Blacklisted,
                1usize,
            >("GetInteractionOrNull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetInteractionOrNull",
                    1usize
                )
            });
        let __cordl_ret: Blacklisted = unsafe {
            method.invoke_unchecked(self, (trigger))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetValueSizeInBytes(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("GetValueSizeInBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetValueSizeInBytes",
                    2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (bindingIndex, controlIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetValueType(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                2usize,
            >("GetValueType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "GetValueType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, (bindingIndex, controlIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasEnabledActions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("HasEnabledActions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "HasEnabledActions", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HookOnBeforeUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("HookOnBeforeUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "HookOnBeforeUpdate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        resolver: crate::UnityEngine::InputSystem::InputBindingResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::InputBindingResolver),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Initialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "Initialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (resolver))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsActionBoundToControlFromDevice(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        actionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputDevice,
                    >,
                    i32,
                ),
                bool,
                2usize,
            >("IsActionBoundToControlFromDevice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "IsActionBoundToControlFromDevice", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (device, actionIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsActiveControl(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("IsActiveControl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "IsActiveControl", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (bindingIndex, controlIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsActuated(
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        threshold: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                    >,
                    f32,
                ),
                bool,
                2usize,
            >("IsActuated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "IsActuated", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (trigger, threshold))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsConflictingInput(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        actionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                    >,
                    i32,
                ),
                bool,
                2usize,
            >("IsConflictingInput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "IsConflictingInput", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (trigger, actionIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsControlEnabled(
        &mut self,
        controlIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), bool, 1usize>("IsControlEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "IsControlEnabled", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (controlIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsUsingDevice(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputDevice,
                >),
                bool,
                1usize,
            >("IsUsingDevice")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "IsUsingDevice", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (device))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyListenersOfActionChange_Il2CppObject1(
        change: crate::UnityEngine::InputSystem::InputActionChange,
        actionOrMapOrAsset: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::InputSystem::InputActionChange,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("NotifyListenersOfActionChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "NotifyListenersOfActionChange", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (change, actionOrMapOrAsset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyListenersOfActionChange_InputActionChange0(
        &mut self,
        change: crate::UnityEngine::InputSystem::InputActionChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::InputActionChange),
                quest_hook::libil2cpp::Void,
                1usize,
            >("NotifyListenersOfActionChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "NotifyListenersOfActionChange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (change))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnBeforeInitialUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("OnBeforeInitialUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "OnBeforeInitialUpdate",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnDeviceChange(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        change: crate::UnityEngine::InputSystem::InputDeviceChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputDevice,
                    >,
                    crate::UnityEngine::InputSystem::InputDeviceChange,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnDeviceChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "OnDeviceChange", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (device, change))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareForBindingReResolution(
        &mut self,
        needFullResolve: bool,
        activeControls: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            >,
        >,
        hasEnabledActions: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputControlList_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("PrepareForBindingReResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "PrepareForBindingReResolution", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (needFullResolve, activeControls, hasEnabledActions),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessButtonState(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        actionIndex: i32,
        bindingStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ProcessButtonState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ProcessButtonState", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (trigger, actionIndex, bindingStatePtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessControlStateChange(
        &mut self,
        mapIndex: i32,
        controlIndex: i32,
        bindingIndex: i32,
        _cordl_time: f64,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    f64,
                    crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("ProcessControlStateChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ProcessControlStateChange",
                    5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (mapIndex, controlIndex, bindingIndex, _cordl_time, eventPtr),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessDefaultInteraction(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        actionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessDefaultInteraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ProcessDefaultInteraction",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (trigger, actionIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessInteractions(
        &mut self,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
        interactionStartIndex: i32,
        interactionCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                    >,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ProcessInteractions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ProcessInteractions",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (trigger, interactionStartIndex, interactionCount),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTimeout(
        &mut self,
        _cordl_time: f64,
        mapIndex: i32,
        controlIndex: i32,
        bindingIndex: i32,
        interactionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f64, i32, i32, i32, i32),
                quest_hook::libil2cpp::Void,
                5usize,
            >("ProcessTimeout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ProcessTimeout", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (_cordl_time, mapIndex, controlIndex, bindingIndex, interactionIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadCompositePartValueAsObject(
        &mut self,
        bindingIndex: i32,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("ReadCompositePartValueAsObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ReadCompositePartValueAsObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (bindingIndex, partNumber))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadCompositePartValue_ByRefMut_TComparer0<TValue, TComparer>(
        &mut self,
        bindingIndex: i32,
        partNumber: i32,
        buttonValuePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        controlIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        comparer: TComparer,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TComparer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    TComparer,
                ),
                TValue,
                5usize,
            >("ReadCompositePartValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ReadCompositePartValue",
                    5usize
                )
            });
        let __cordl_ret: TValue = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (bindingIndex, partNumber, buttonValuePtr, controlIndex, comparer),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadCompositePartValue_i32_1(
        &mut self,
        bindingIndex: i32,
        partNumber: i32,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                ),
                bool,
                4usize,
            >("ReadCompositePartValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ReadCompositePartValue",
                    4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(self, (bindingIndex, partNumber, buffer, bufferSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueAsButton(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("ReadValueAsButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ReadValueAsButton", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (bindingIndex, controlIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueAsObject(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
        ignoreComposites: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("ReadValueAsObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ReadValueAsObject", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method
                .invoke_unchecked(self, (bindingIndex, controlIndex, ignoreComposites))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue_Il2CppObject_i32__cordl_bool0(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: i32,
        ignoreComposites: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("ReadValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ReadValue", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (bindingIndex, controlIndex, buffer, bufferSize, ignoreComposites),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue__cordl_bool1<TValue>(
        &mut self,
        bindingIndex: i32,
        controlIndex: i32,
        ignoreComposites: bool,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32, bool), TValue, 3usize>("ReadValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ReadValue", 3usize
                )
            });
        let __cordl_ret: TValue = unsafe {
            method
                .invoke_unchecked(self, (bindingIndex, controlIndex, ignoreComposites))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveMapFromGlobalList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RemoveMapFromGlobalList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "RemoveMapFromGlobalList",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetActionState(
        &mut self,
        actionIndex: i32,
        toPhase: crate::UnityEngine::InputSystem::InputActionPhase,
        hardReset: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, crate::UnityEngine::InputSystem::InputActionPhase, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ResetActionState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ResetActionState", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (actionIndex, toPhase, hardReset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetActionStatesDrivenBy(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputDevice,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ResetActionStatesDrivenBy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ResetActionStatesDrivenBy",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (device))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetGlobals() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ResetGlobals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ResetGlobals", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetInteractionState(
        &mut self,
        interactionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ResetInteractionState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "ResetInteractionState",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (interactionIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetInteractionStateAndCancelIfNecessary(
        &mut self,
        mapIndex: i32,
        bindingIndex: i32,
        interactionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ResetInteractionStateAndCancelIfNecessary")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ResetInteractionStateAndCancelIfNecessary", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (mapIndex, bindingIndex, interactionIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RestoreActionStatesAfterReResolvingBindings(
        &mut self,
        oldState: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
        activeControls: crate::UnityEngine::InputSystem::InputControlList_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        >,
        isFullResolve: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
                    crate::UnityEngine::InputSystem::InputControlList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("RestoreActionStatesAfterReResolvingBindings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "RestoreActionStatesAfterReResolvingBindings", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (oldState, activeControls, isFullResolve))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveAndResetState() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::ISavedState,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::Utilities::ISavedState,
                >,
                0usize,
            >("SaveAndResetState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "SaveAndResetState", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::ISavedState,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetControlEnabled(
        &mut self,
        controlIndex: i32,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetControlEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "SetControlEnabled", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (controlIndex, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetInitialStateCheckPending_Il2CppObject1(
        &mut self,
        bindingStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetInitialStateCheckPending")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "SetInitialStateCheckPending", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bindingStatePtr, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetInitialStateCheckPending_i32_0(
        &mut self,
        actionIndex: i32,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetInitialStateCheckPending")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "SetInitialStateCheckPending", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (actionIndex, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTotalTimeoutCompletionTime(
        &mut self,
        seconds: f32,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTotalTimeoutCompletionTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "SetTotalTimeoutCompletionTime", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (seconds, trigger))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldIgnoreInputOnCompositeBinding(
        binding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        eventPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                bool,
                2usize,
            >("ShouldIgnoreInputOnCompositeBinding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ShouldIgnoreInputOnCompositeBinding", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (binding, eventPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SplitUpMapAndControlAndBindingIndex(
        &mut self,
        mapControlAndBindingIndex: i64,
        mapIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        controlIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        bindingIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i64,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SplitUpMapAndControlAndBindingIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "SplitUpMapAndControlAndBindingIndex", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (mapControlAndBindingIndex, mapIndex, controlIndex, bindingIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartTimeout(
        &mut self,
        seconds: f32,
        trigger: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionState_TriggerState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputActionState_TriggerState,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("StartTimeout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "StartTimeout", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (seconds, trigger))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StopTimeout(
        &mut self,
        interactionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("StopTimeout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "StopTimeout", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (interactionIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_ICloneable_Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("System.ICloneable.Clone")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "System.ICloneable.Clone",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToCombinedMapAndControlAndBindingIndex(
        &mut self,
        mapIndex: i32,
        controlIndex: i32,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                i64,
                3usize,
            >("ToCombinedMapAndControlAndBindingIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ToCombinedMapAndControlAndBindingIndex", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked(self, (mapIndex, controlIndex, bindingIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnhookOnBeforeUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UnhookOnBeforeUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "UnhookOnBeforeUpdate",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateChangeMonitor_NotifyControlStateChanged(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        _cordl_time: f64,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        mapControlAndBindingIndex: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
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
            >(
                "UnityEngine.InputSystem.LowLevel.IInputStateChangeMonitor.NotifyControlStateChanged",
            )
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "UnityEngine.InputSystem.LowLevel.IInputStateChangeMonitor.NotifyControlStateChanged",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (control, _cordl_time, eventPtr, mapControlAndBindingIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateChangeMonitor_NotifyTimerExpired(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        _cordl_time: f64,
        mapControlAndBindingIndex: i64,
        interactionIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
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
            >(
                "UnityEngine.InputSystem.LowLevel.IInputStateChangeMonitor.NotifyTimerExpired",
            )
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "UnityEngine.InputSystem.LowLevel.IInputStateChangeMonitor.NotifyTimerExpired",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (control, _cordl_time, mapControlAndBindingIndex, interactionIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_actionStates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_actionStates")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_actionStates", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_bindingStates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_bindingStates")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_bindingStates", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controlGroupingAndComplexity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_controlGroupingAndComplexity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_controlGroupingAndComplexity", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controlIndexToBindingIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_controlIndexToBindingIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_controlIndexToBindingIndex", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controlMagnitudes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_controlMagnitudes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_controlMagnitudes",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_enabledControls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_enabledControls")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_enabledControls",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_interactionStates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_interactionStates")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_interactionStates",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isProcessingControlStateChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isProcessingControlStateChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_isProcessingControlStateChange", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_mapIndices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_mapIndices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_mapIndices", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalActionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_totalActionCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_totalActionCount",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalBindingCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_totalBindingCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_totalBindingCount",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalCompositeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_totalCompositeCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_totalCompositeCount",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalControlCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_totalControlCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_totalControlCount",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalInteractionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_totalInteractionCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_totalInteractionCount",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalMapCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_totalMapCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState as
                    quest_hook::libil2cpp::Type > ::class(), "get_totalMapCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl AsRef<crate::System::ICloneable>
for crate::UnityEngine::InputSystem::InputActionState {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl AsMut<crate::System::ICloneable>
for crate::UnityEngine::InputSystem::InputActionState {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputActionState {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputActionState {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor>
for crate::UnityEngine::InputSystem::InputActionState {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor>
for crate::UnityEngine::InputSystem::InputActionState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputActionState_ActionMapIndices {
    pub actionStartIndex: i32,
    pub actionCount: i32,
    pub controlStartIndex: i32,
    pub controlCount: i32,
    pub bindingStartIndex: i32,
    pub bindingCount: i32,
    pub interactionStartIndex: i32,
    pub interactionCount: i32,
    pub processorStartIndex: i32,
    pub processorCount: i32,
    pub compositeStartIndex: i32,
    pub compositeCount: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionState/ActionMapIndices";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+ActionMapIndices")]
impl crate::UnityEngine::InputSystem::InputActionState_ActionMapIndices {}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputActionState_BindingState {
    padding: quest_hook::libil2cpp::ValueTypePadding<32usize>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionState_BindingState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionState/BindingState";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputActionState_BindingState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputActionState_BindingState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputActionState_BindingState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputActionState_BindingState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_BindingState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState")]
impl crate::UnityEngine::InputSystem::InputActionState_BindingState {
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+BindingState+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags;
    pub fn get_actionIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_actionIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_actionIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_chainsWithNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_chainsWithNext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_chainsWithNext", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_compositeOrCompositeBindingIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_compositeOrCompositeBindingIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_compositeOrCompositeBindingIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controlCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_controlCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_controlCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controlStartIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_controlStartIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_controlStartIndex",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags,
                0usize,
            >("get_flags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_flags", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_initialStateCheckPending(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_initialStateCheckPending")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_initialStateCheckPending", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_interactionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_interactionCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_interactionCount",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_interactionStartIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_interactionStartIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_interactionStartIndex",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isComposite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isComposite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isComposite", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isEndOfChain(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isEndOfChain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isEndOfChain", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isPartOfChain(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isPartOfChain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isPartOfChain", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isPartOfComposite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isPartOfComposite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isPartOfComposite",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_mapIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_mapIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_mapIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_partIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_partIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_partIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pressTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f64, 0usize>("get_pressTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_pressTime", 0usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_processorCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_processorCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_processorCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_processorStartIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_processorStartIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "get_processorStartIndex",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_triggerEventIdForComposite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_triggerEventIdForComposite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_triggerEventIdForComposite", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_wantsInitialStateCheck(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_wantsInitialStateCheck")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_wantsInitialStateCheck", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_actionIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_actionIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_actionIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_chainsWithNext(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_chainsWithNext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_chainsWithNext", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_compositeOrCompositeBindingIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_compositeOrCompositeBindingIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "set_compositeOrCompositeBindingIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_controlCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_controlCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_controlCount", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_controlStartIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_controlStartIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_controlStartIndex",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_flags(
        &mut self,
        value: crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::BindingState_InputActionState_Flags),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_flags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_flags", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_initialStateCheckPending(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_initialStateCheckPending")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "set_initialStateCheckPending", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_interactionCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_interactionCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_interactionCount",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_interactionStartIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_interactionStartIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_interactionStartIndex",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_isComposite(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_isComposite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_isComposite", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_isEndOfChain(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_isEndOfChain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_isEndOfChain", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_isPartOfComposite(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_isPartOfComposite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_isPartOfComposite",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_mapIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_mapIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_mapIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_partIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_partIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_partIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_pressTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64), quest_hook::libil2cpp::Void, 1usize>("set_pressTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_pressTime", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_processorCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_processorCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_processorCount", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_processorStartIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_processorStartIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(), "set_processorStartIndex",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_triggerEventIdForComposite(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_triggerEventIdForComposite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "set_triggerEventIdForComposite", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_wantsInitialStateCheck(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_BindingState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_wantsInitialStateCheck")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_BindingState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "set_wantsInitialStateCheck", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputActionState_GlobalState {
    pub globalList: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::System::Runtime::InteropServices::GCHandle,
    >,
    pub onActionChange: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                crate::UnityEngine::InputSystem::InputActionChange,
            >,
        >,
    >,
    pub onActionControlsChanged: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionState_GlobalState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionState/GlobalState";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputActionState_GlobalState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputActionState_GlobalState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputActionState_GlobalState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputActionState_GlobalState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_GlobalState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+GlobalState")]
impl crate::UnityEngine::InputSystem::InputActionState_GlobalState {}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputActionState_InteractionState {
    padding: quest_hook::libil2cpp::ValueTypePadding<48usize>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionState_InteractionState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionState/InteractionState";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputActionState_InteractionState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputActionState_InteractionState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputActionState_InteractionState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputActionState_InteractionState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_InteractionState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState")]
impl crate::UnityEngine::InputSystem::InputActionState_InteractionState {
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::InteractionState_InputActionState_Flags;
    pub fn get_isTimerRunning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isTimerRunning")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "get_isTimerRunning",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_performedTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f64, 0usize>("get_performedTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "get_performedTime",
                    0usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_phase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionPhase,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::InputSystem::InputActionPhase,
                0usize,
            >("get_phase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "get_phase", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionPhase = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_startTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f64, 0usize>("get_startTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "get_startTime", 0usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_timerDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_timerDuration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "get_timerDuration",
                    0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_timerMonitorIndex(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("get_timerMonitorIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "get_timerMonitorIndex",
                    0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_timerStartTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f64, 0usize>("get_timerStartTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "get_timerStartTime",
                    0usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalTimeoutCompletionDone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_totalTimeoutCompletionDone")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_totalTimeoutCompletionDone", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalTimeoutCompletionTimeRemaining(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_totalTimeoutCompletionTimeRemaining")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_totalTimeoutCompletionTimeRemaining", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_triggerControlIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_triggerControlIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_triggerControlIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isTimerRunning(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_isTimerRunning")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "set_isTimerRunning",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_performedTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_performedTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "set_performedTime",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_phase(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionPhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::InputActionPhase),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_phase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "set_phase", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_startTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64), quest_hook::libil2cpp::Void, 1usize>("set_startTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "set_startTime", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_timerDuration(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_timerDuration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "set_timerDuration",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_timerMonitorIndex(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_timerMonitorIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "set_timerMonitorIndex",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_timerStartTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_timerStartTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(), "set_timerStartTime",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_totalTimeoutCompletionDone(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_totalTimeoutCompletionDone")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(),
                    "set_totalTimeoutCompletionDone", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_totalTimeoutCompletionTimeRemaining(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_totalTimeoutCompletionTimeRemaining")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(),
                    "set_totalTimeoutCompletionTimeRemaining", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_triggerControlIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_InteractionState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_triggerControlIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_InteractionState
                    as quest_hook::libil2cpp::Type > ::class(),
                    "set_triggerControlIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputActionState_TriggerState {
    padding: quest_hook::libil2cpp::ValueTypePadding<48usize>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionState_TriggerState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionState/TriggerState";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputActionState_TriggerState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputActionState_TriggerState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputActionState_TriggerState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputActionState_TriggerState {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_TriggerState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState")]
impl crate::UnityEngine::InputSystem::InputActionState_TriggerState {
    pub const kMaxNumBindings: i32 = 65535i32;
    pub const kMaxNumControls: i32 = 65535i32;
    pub const kMaxNumMaps: i32 = 255i32;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags;
    pub fn get_bindingIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_bindingIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_bindingIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controlIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_controlIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_controlIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags,
                0usize,
            >("get_flags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_flags", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasMultipleConcurrentActuations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_hasMultipleConcurrentActuations")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_hasMultipleConcurrentActuations", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_haveMagnitude(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_haveMagnitude")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_haveMagnitude", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_inProcessing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_inProcessing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_inProcessing", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_interactionIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_interactionIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_interactionIndex",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isButton", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isCanceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isCanceled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isCanceled", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isDisabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isDisabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isDisabled", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isPassThrough(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isPassThrough")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isPassThrough", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isPerformed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isPerformed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isPerformed", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isPressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isPressed", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isStarted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isStarted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isStarted", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isWaiting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isWaiting")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_isWaiting", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_lastCanceledInUpdate(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u32, 0usize>("get_lastCanceledInUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_lastCanceledInUpdate",
                    0usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_lastPerformedInUpdate(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u32, 0usize>("get_lastPerformedInUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_lastPerformedInUpdate",
                    0usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_magnitude(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_magnitude")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_magnitude", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_mapIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_mapIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_mapIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_mayNeedConflictResolution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_mayNeedConflictResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_mayNeedConflictResolution", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_phase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionPhase,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::InputSystem::InputActionPhase,
                0usize,
            >("get_phase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_phase", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionPhase = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_pressedInUpdate(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u32, 0usize>("get_pressedInUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_pressedInUpdate",
                    0usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_releasedInUpdate(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u32, 0usize>("get_releasedInUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_releasedInUpdate",
                    0usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_startTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f64, 0usize>("get_startTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_startTime", 0usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f64, 0usize>("get_time")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "get_time", 0usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_bindingIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_bindingIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_bindingIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_controlIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_controlIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_controlIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_flags(
        &mut self,
        value: crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_flags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_flags", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_hasMultipleConcurrentActuations(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_hasMultipleConcurrentActuations")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "set_hasMultipleConcurrentActuations", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_inProcessing(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_inProcessing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_inProcessing", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_interactionIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_interactionIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_interactionIndex",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_isButton(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_isButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_isButton", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_isPassThrough(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_isPassThrough")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_isPassThrough", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_isPressed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_isPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_isPressed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_lastCanceledInUpdate(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_lastCanceledInUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_lastCanceledInUpdate",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_lastPerformedInUpdate(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_lastPerformedInUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_lastPerformedInUpdate",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_magnitude(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_magnitude")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_magnitude", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_mapIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_mapIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_mapIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_mayNeedConflictResolution(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_mayNeedConflictResolution")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(),
                    "set_mayNeedConflictResolution", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_phase(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionPhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::InputActionPhase),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_phase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_phase", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_pressedInUpdate(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_pressedInUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_pressedInUpdate",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_releasedInUpdate(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_releasedInUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_releasedInUpdate",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_startTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64), quest_hook::libil2cpp::Void, 1usize>("set_startTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_startTime", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_time(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_TriggerState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64), quest_hook::libil2cpp::Void, 1usize>("set_time")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_TriggerState as
                    quest_hook::libil2cpp::Type > ::class(), "set_time", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputActionState_UnmanagedMemory {
    pub basePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub mapCount: i32,
    pub actionCount: i32,
    pub interactionCount: i32,
    pub bindingCount: i32,
    pub controlCount: i32,
    pub compositeCount: i32,
    pub actionStates: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub bindingStates: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub interactionStates: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub controlMagnitudes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub compositeMagnitudes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub enabledControls: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub actionBindingIndicesAndCounts: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub actionBindingIndices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub controlIndexToBindingIndex: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub controlGroupingAndComplexity: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub controlGroupingInitialized: bool,
    pub mapIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionState/UnmanagedMemory";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
impl crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
    pub fn Allocate(
        &mut self,
        mapCount: i32,
        actionCount: i32,
        bindingCount: i32,
        controlCount: i32,
        interactionCount: i32,
        compositeCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, i32, i32),
                quest_hook::libil2cpp::Void,
                6usize,
            >("Allocate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as
                    quest_hook::libil2cpp::Type > ::class(), "Allocate", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        mapCount,
                        actionCount,
                        bindingCount,
                        controlCount,
                        interactionCount,
                        compositeCount,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
                0usize,
            >("Clone")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as
                    quest_hook::libil2cpp::Type > ::class(), "Clone", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyDataFrom(
        &mut self,
        memory: crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CopyDataFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as
                    quest_hook::libil2cpp::Type > ::class(), "CopyDataFrom", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (memory))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isAllocated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isAllocated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as
                    quest_hook::libil2cpp::Type > ::class(), "get_isAllocated", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_sizeInBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::InputSystem::InputActionState_UnmanagedMemory as
                    quest_hook::libil2cpp::Type > ::class(), "get_sizeInBytes", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+UnmanagedMemory")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::InputActionState_UnmanagedMemory {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InteractionState_InputActionState_Flags {
    #[default]
    TimerRunning = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState+Flags")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InteractionState_InputActionState_Flags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionState/InteractionState/Flags";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState+Flags")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InteractionState_InputActionState_Flags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState+Flags")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InteractionState_InputActionState_Flags {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState+Flags")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InteractionState_InputActionState_Flags {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+InteractionState+Flags")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InteractionState_InputActionState_Flags {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TriggerState_InputActionState_Flags {
    #[default]
    Button = 32i32,
    HasMultipleConcurrentActuations = 8i32,
    HaveMagnitude = 1i32,
    InProcessing = 16i32,
    MayNeedConflictResolution = 4i32,
    PassThrough = 2i32,
    Pressed = 64i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState+Flags")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputActionState/TriggerState/Flags";
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState+Flags")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState+Flags")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState+Flags")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags {
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
#[cfg(feature = "UnityEngine+InputSystem+InputActionState+TriggerState+Flags")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::TriggerState_InputActionState_Flags {
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
