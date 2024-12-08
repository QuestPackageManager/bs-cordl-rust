#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper
    => "UnityEngine.InputSystem"
    ."InputActionRebindingExtensions/DeferBindingResolutionWrapper"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    pub fn Acquire(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Acquire", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation+Flags"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RebindingOperation_Flags {
    AddNewBinding = 256i32,
    Canceled = 4i32,
    Completed = 2i32,
    DontGeneralizePathOfSelectedControl = 128i32,
    DontIgnoreNoisyControls = 64i32,
    OnAfterUpdateHooked = 16i32,
    OnEventHooked = 8i32,
    Started = 1i32,
    SuppressMatchingEvents = 512i32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation+Flags"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::RebindingOperation_Flags => "UnityEngine.InputSystem"
    ."InputActionRebindingExtensions/RebindingOperation/Flags"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionRebindingExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions {
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+__c__DisplayClass25_0"
    )]
    pub type __c__DisplayClass25_0 = crate::UnityEngine::InputSystem::InputActionRebindingExtensions___c__DisplayClass25_0;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterOverride"
    )]
    pub type ParameterOverride = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride;
    #[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+Parameter")]
    pub type Parameter = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
    )]
    pub type ParameterEnumerator = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+DeferBindingResolutionWrapper"
    )]
    pub type DeferBindingResolutionWrapper = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_DeferBindingResolutionWrapper;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
    )]
    pub type ParameterEnumerable = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
    )]
    pub type RebindingOperation = crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation;
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+Parameter")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionRebindingExtensions_Parameter {
    pub instance: *mut crate::System::Object,
    pub field: *mut crate::System::Reflection::FieldInfo,
    pub bindingIndex: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+Parameter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions/Parameter"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+Parameter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+Parameter")]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter {}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionRebindingExtensions_ParameterEnumerable {
    pub m_State: *mut crate::UnityEngine::InputSystem::InputActionState,
    pub m_Parameter: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
    pub m_MapIndex: i32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions/ParameterEnumerable"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerable"
)]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerable {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IEnumerable_UnityEngine_InputSystem_InputActionRebindingExtensions_Parameter__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
        >,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IEnumerable<UnityEngine.InputSystem.InputActionRebindingExtensions.Parameter>.GetEnumerator",
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
    pub fn _ctor(
        &mut self,
        state: *mut crate::UnityEngine::InputSystem::InputActionState,
        parameter: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        mapIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (state, parameter, mapIndex),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionRebindingExtensions_ParameterEnumerator {
    pub m_State: *mut crate::UnityEngine::InputSystem::InputActionState,
    pub m_MapIndex: i32,
    pub m_BindingCurrentIndex: i32,
    pub m_BindingEndIndex: i32,
    pub m_InteractionCurrentIndex: i32,
    pub m_InteractionEndIndex: i32,
    pub m_ProcessorCurrentIndex: i32,
    pub m_ProcessorEndIndex: i32,
    pub m_BindingMask: crate::UnityEngine::InputSystem::InputBinding,
    pub m_ObjectType: *mut crate::System::Type,
    pub m_ParameterName: *mut crate::System::String,
    pub m_MayBeInteraction: bool,
    pub m_MayBeProcessor: bool,
    pub m_MayBeComposite: bool,
    pub m_CurrentBindingIsComposite: bool,
    pub m_CurrentObject: *mut crate::System::Object,
    pub m_CurrentParameter: *mut crate::System::Reflection::FieldInfo,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions/ParameterEnumerator"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterEnumerator"
)]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterEnumerator {
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
    pub fn FindParameter(
        &mut self,
        instance: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FindParameter",
            (instance),
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
    pub fn MoveToNextBinding(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveToNextBinding",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MoveToNextInteraction(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveToNextInteraction",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MoveToNextProcessor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveToNextProcessor",
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
    pub fn _ctor(
        &mut self,
        state: *mut crate::UnityEngine::InputSystem::InputActionState,
        parameter: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride,
        mapIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (state, parameter, mapIndex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionRebindingExtensions_Parameter = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterOverride"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionRebindingExtensions_ParameterOverride {
    pub objectRegistrationName: *mut crate::System::String,
    pub parameter: *mut crate::System::String,
    pub bindingMask: crate::UnityEngine::InputSystem::InputBinding,
    pub value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterOverride"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions/ParameterOverride"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterOverride"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+ParameterOverride"
)]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_ParameterOverride {
    pub fn _ctor_InputBinding_PrimitiveValue0(
        &mut self,
        parameterName: *mut crate::System::String,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (parameterName, bindingMask, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_InputBinding_PrimitiveValue1(
        &mut self,
        objectRegistrationName: *mut crate::System::String,
        parameterName: *mut crate::System::String,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
        value: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (objectRegistrationName, parameterName, bindingMask, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_objectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_ret: *mut crate::System::Type = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_objectType",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InputActionRebindingExtensions_RebindingOperation {
    __cordl_parent: crate::System::Object,
    pub m_ActionToRebind: *mut crate::UnityEngine::InputSystem::InputAction,
    pub m_BindingMask: crate::System::Nullable_1<
        crate::UnityEngine::InputSystem::InputBinding,
    >,
    pub m_ControlType: *mut crate::System::Type,
    pub m_ExpectedLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    pub m_IncludePathCount: i32,
    pub m_IncludePaths: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub m_ExcludePathCount: i32,
    pub m_ExcludePaths: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub m_TargetBindingIndex: i32,
    pub m_BindingGroupForNewBinding: *mut crate::System::String,
    pub m_CancelBinding: *mut crate::System::String,
    pub m_MagnitudeThreshold: f32,
    pub m_Scores: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub m_Magnitudes: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub m_LastMatchTime: f64,
    pub m_StartTime: f64,
    pub m_Timeout: f32,
    pub m_WaitSecondsAfterMatch: f32,
    pub m_Candidates: crate::UnityEngine::InputSystem::InputControlList_1<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
    pub m_OnComplete: *mut crate::System::Action_1<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    >,
    pub m_OnCancel: *mut crate::System::Action_1<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    >,
    pub m_OnPotentialMatch: *mut crate::System::Action_1<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    >,
    pub m_OnGeneratePath: *mut crate::System::Func_2<
        *mut crate::UnityEngine::InputSystem::InputControl,
        *mut crate::System::String,
    >,
    pub m_OnComputeScore: *mut crate::System::Func_3<
        *mut crate::UnityEngine::InputSystem::InputControl,
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        f32,
    >,
    pub m_OnApplyBinding: *mut crate::System::Action_2<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        *mut crate::System::String,
    >,
    pub m_OnEventDelegate: *mut crate::System::Action_2<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        *mut crate::UnityEngine::InputSystem::InputDevice,
    >,
    pub m_OnAfterUpdateDelegate: *mut crate::System::Action,
    pub m_LayoutCache: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Cache,
    pub m_PathBuilder: *mut crate::System::Text::StringBuilder,
    pub m_Flags: crate::UnityEngine::InputSystem::RebindingOperation_Flags,
    pub m_StartingActuations: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::InputSystem::InputControl,
        f32,
    >,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation =>
    "UnityEngine.InputSystem"."InputActionRebindingExtensions/RebindingOperation"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
impl crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation {
    pub const kDefaultMagnitudeThreshold: f32 = 0.2f32;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation+__c__DisplayClass32_0"
    )]
    pub type __c__DisplayClass32_0 = crate::UnityEngine::InputSystem::RebindingOperation___c__DisplayClass32_0;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation+Flags"
    )]
    pub type Flags = crate::UnityEngine::InputSystem::RebindingOperation_Flags;
    pub fn AddCandidate(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        score: f32,
        magnitude: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCandidate", (control, score, magnitude))?;
        Ok(__cordl_ret)
    }
    pub fn Cancel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cancel", ())?;
        Ok(__cordl_ret)
    }
    pub fn Complete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GeneratePathForControl(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GeneratePathForControl", (control))?;
        Ok(__cordl_ret)
    }
    pub fn HookOnAfterUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HookOnAfterUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn HookOnEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HookOnEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnAfterUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnApplyBinding(
        &mut self,
        callback: *mut crate::System::Action_2<
            *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("OnApplyBinding", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn OnCancel_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCancel", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnCancel_Action_1_0(
        &mut self,
        callback: *mut crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("OnCancel", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn OnComplete_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnComplete", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnComplete_Action_1_0(
        &mut self,
        callback: *mut crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("OnComplete", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn OnComputeScore(
        &mut self,
        callback: *mut crate::System::Func_3<
            *mut crate::UnityEngine::InputSystem::InputControl,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            f32,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("OnComputeScore", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn OnEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEvent", (eventPtr, device))?;
        Ok(__cordl_ret)
    }
    pub fn OnGeneratePath(
        &mut self,
        callback: *mut crate::System::Func_2<
            *mut crate::UnityEngine::InputSystem::InputControl,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("OnGeneratePath", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn OnMatchWaitForAnother(
        &mut self,
        seconds: f32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("OnMatchWaitForAnother", (seconds))?;
        Ok(__cordl_ret)
    }
    pub fn OnPotentialMatch(
        &mut self,
        callback: *mut crate::System::Action_1<
            *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("OnPotentialMatch", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveCandidate(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCandidate", (control))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetAfterMatchCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetAfterMatchCompleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn SortCandidatesByScore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortCandidatesByScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn ThrowIfRebindInProgress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfRebindInProgress", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnhookOnAfterUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnhookOnAfterUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnhookOnEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnhookOnEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn WithAction(
        &mut self,
        action: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithAction", (action))?;
        Ok(__cordl_ret)
    }
    pub fn WithBindingGroup(
        &mut self,
        group: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithBindingGroup", (group))?;
        Ok(__cordl_ret)
    }
    pub fn WithBindingMask(
        &mut self,
        bindingMask: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithBindingMask", (bindingMask))?;
        Ok(__cordl_ret)
    }
    pub fn WithCancelingThrough_InputControl1(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithCancelingThrough", (control))?;
        Ok(__cordl_ret)
    }
    pub fn WithCancelingThrough_String0(
        &mut self,
        binding: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithCancelingThrough", (binding))?;
        Ok(__cordl_ret)
    }
    pub fn WithControlsExcluding(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithControlsExcluding", (path))?;
        Ok(__cordl_ret)
    }
    pub fn WithControlsHavingToMatchPath(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithControlsHavingToMatchPath", (path))?;
        Ok(__cordl_ret)
    }
    pub fn WithExpectedControlType_2<TControl>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    >
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithExpectedControlType", ())?;
        Ok(__cordl_ret)
    }
    pub fn WithExpectedControlType_String0(
        &mut self,
        layoutName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithExpectedControlType", (layoutName))?;
        Ok(__cordl_ret)
    }
    pub fn WithExpectedControlType_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithExpectedControlType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn WithMagnitudeHavingToBeGreaterThan(
        &mut self,
        magnitude: f32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithMagnitudeHavingToBeGreaterThan", (magnitude))?;
        Ok(__cordl_ret)
    }
    pub fn WithMatchingEventsBeingSuppressed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithMatchingEventsBeingSuppressed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WithRebindAddingNewBinding(
        &mut self,
        group: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithRebindAddingNewBinding", (group))?;
        Ok(__cordl_ret)
    }
    pub fn WithTargetBinding(
        &mut self,
        bindingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithTargetBinding", (bindingIndex))?;
        Ok(__cordl_ret)
    }
    pub fn WithTimeout(
        &mut self,
        timeInSeconds: f32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithTimeout", (timeInSeconds))?;
        Ok(__cordl_ret)
    }
    pub fn WithoutGeneralizingPathOfSelectedControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithoutGeneralizingPathOfSelectedControl", ())?;
        Ok(__cordl_ret)
    }
    pub fn WithoutIgnoringNoisyControls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation = __cordl_object
            .invoke("WithoutIgnoringNoisyControls", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_action(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = __cordl_object
            .invoke("get_action", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bindingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputBinding>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        > = __cordl_object.invoke("get_bindingMask", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_canceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canceled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_candidates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlList_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlList_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        > = __cordl_object.invoke("get_candidates", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_completed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_completed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_expectedControlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_expectedControlType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_magnitudes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<f32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            f32,
        > = __cordl_object.invoke("get_magnitudes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scores(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<f32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            f32,
        > = __cordl_object.invoke("get_scores", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputControl = __cordl_object
            .invoke("get_selectedControl", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_startTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_startTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_started(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_started", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_timeout(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timeout", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputActionRebindingExtensions+RebindingOperation"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputActionRebindingExtensions_RebindingOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}