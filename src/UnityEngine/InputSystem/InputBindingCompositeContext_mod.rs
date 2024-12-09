#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputBindingCompositeContext {
    pub m_State: *mut crate::UnityEngine::InputSystem::InputActionState,
    pub m_BindingIndex: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputBindingCompositeContext => "UnityEngine.InputSystem"
    ."InputBindingCompositeContext"
);
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputBindingCompositeContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext")]
impl crate::UnityEngine::InputSystem::InputBindingCompositeContext {
    #[cfg(
        feature = "UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
    )]
    pub type DefaultComparer_1<TValue: quest_hook::libil2cpp::Type> = crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<
        TValue,
    >;
    #[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
    pub type PartBinding = crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputBindingCompositeContext+_get_controls_d__2"
    )]
    pub type _get_controls_d__2 = crate::UnityEngine::InputSystem::InputBindingCompositeContext__get_controls_d__2;
    pub fn EvaluateMagnitude(
        &mut self,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "EvaluateMagnitude",
            (partNumber),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetPressTime(
        &mut self,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPressTime",
            (partNumber),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ReadValueAsButton(
        &mut self,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValueAsButton",
            (partNumber),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ReadValueAsObject(
        &mut self,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValueAsObject",
            (partNumber),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ReadValue_ByRefMut1<TValue>(
        &mut self,
        partNumber: i32,
        sourceControl: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValue",
            (partNumber, sourceControl),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ReadValue_ByRefMut_TComparer3<TValue, TComparer>(
        &mut self,
        partNumber: i32,
        sourceControl: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
        comparer: TComparer,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TComparer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValue",
            (partNumber, sourceControl, comparer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ReadValue_Il2CppObject_i32_4(
        &mut self,
        partNumber: i32,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValue",
            (partNumber, buffer, bufferSize),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ReadValue_TComparer2<TValue, TComparer>(
        &mut self,
        partNumber: i32,
        comparer: TComparer,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TComparer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValue",
            (partNumber, comparer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ReadValue_i32_0<TValue>(
        &mut self,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValue",
            (partNumber),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_controls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding,
        >,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_controls", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputBindingCompositeContext_DefaultComparer_1<
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1 < TValue >
    => "UnityEngine.InputSystem"."InputBindingCompositeContext/DefaultComparer`1<TValue>"
    < TValue >
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<
    TValue,
> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<
    TValue,
> {
    pub fn Compare(&mut self, x: TValue, y: TValue) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Compare",
            (x, y),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputBindingCompositeContext_PartBinding {
    pub _part_k__BackingField: i32,
    pub _control_k__BackingField: *mut crate::UnityEngine::InputSystem::InputControl,
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding =>
    "UnityEngine.InputSystem"."InputBindingCompositeContext/PartBinding"
);
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
impl crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding {
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
    pub fn get_part(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_part",
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
    pub fn set_part(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_part",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
