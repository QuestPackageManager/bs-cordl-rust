#[cfg(feature = "UnityEngine+EventSystems+EventTriggerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventTriggerType {
    #[default]
    BeginDrag = 13i32,
    Cancel = 16i32,
    Deselect = 10i32,
    Drag = 5i32,
    Drop = 6i32,
    EndDrag = 14i32,
    InitializePotentialDrag = 12i32,
    Move = 11i32,
    PointerClick = 4i32,
    PointerDown = 2i32,
    PointerEnter = 0i32,
    PointerExit = 1i32,
    PointerUp = 3i32,
    Scroll = 7i32,
    Select = 9i32,
    Submit = 15i32,
    UpdateSelected = 8i32,
}
#[cfg(feature = "UnityEngine+EventSystems+EventTriggerType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::EventSystems::EventTriggerType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "EventTriggerType";
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
#[cfg(feature = "UnityEngine+EventSystems+EventTriggerType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::EventSystems::EventTriggerType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTriggerType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::EventSystems::EventTriggerType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+EventSystems+EventTriggerType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::EventSystems::EventTriggerType {
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
#[cfg(feature = "UnityEngine+EventSystems+EventTriggerType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::EventSystems::EventTriggerType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
