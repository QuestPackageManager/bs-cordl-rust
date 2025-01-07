#[cfg(feature = "UnityEngine+UIElements+ManipulatorActivationFilter")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ManipulatorActivationFilter {
    pub _button_k__BackingField: crate::UnityEngine::UIElements::MouseButton,
    pub _modifiers_k__BackingField: crate::UnityEngine::EventModifiers,
    pub _clickCount_k__BackingField: i32,
}
#[cfg(feature = "UnityEngine+UIElements+ManipulatorActivationFilter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::ManipulatorActivationFilter {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "ManipulatorActivationFilter";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::ManipulatorActivationFilter {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::ManipulatorActivationFilter {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::ManipulatorActivationFilter {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::ManipulatorActivationFilter {
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
#[cfg(feature = "UnityEngine+UIElements+ManipulatorActivationFilter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::ManipulatorActivationFilter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ManipulatorActivationFilter")]
impl crate::UnityEngine::UIElements::ManipulatorActivationFilter {
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ManipulatorActivationFilter1(
        &mut self,
        other: crate::UnityEngine::UIElements::ManipulatorActivationFilter,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn HasModifiers_IMouseEvent0(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IMouseEvent>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasModifiers",
            (e),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn HasModifiers_IPointerEvent1(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPointerEvent>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasModifiers",
            (e),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchModifiers(
        &mut self,
        alt: bool,
        ctrl: bool,
        shift: bool,
        command: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MatchModifiers",
            (alt, ctrl, shift, command),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Matches_IMouseEvent0(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IMouseEvent>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Matches",
            (e),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Matches_IPointerEvent1(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPointerEvent>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Matches",
            (e),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_button(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::MouseButton> {
        let __cordl_ret: crate::UnityEngine::UIElements::MouseButton = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_button",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clickCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_clickCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_modifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::EventModifiers> {
        let __cordl_ret: crate::UnityEngine::EventModifiers = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_modifiers",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_button(
        &mut self,
        value: crate::UnityEngine::UIElements::MouseButton,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_button",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_modifiers(
        &mut self,
        value: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_modifiers",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ManipulatorActivationFilter")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::ManipulatorActivationFilter,
    >,
> for crate::UnityEngine::UIElements::ManipulatorActivationFilter {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::ManipulatorActivationFilter,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+ManipulatorActivationFilter")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::ManipulatorActivationFilter,
    >,
> for crate::UnityEngine::UIElements::ManipulatorActivationFilter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::ManipulatorActivationFilter,
    > {
        todo!()
    }
}
