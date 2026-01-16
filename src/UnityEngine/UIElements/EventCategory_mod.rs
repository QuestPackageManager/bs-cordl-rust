#[cfg(feature = "cordl_class_UnityEngine+UIElements+EventCategory")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum EventCategory {
    #[default]
    Bind = 10i32,
    ChangePanel = 12i32,
    ChangeValue = 9i32,
    Command = 15i32,
    Default = 0i32,
    DragAndDrop = 17i32,
    EnterLeave = 4i32,
    EnterLeaveWindow = 5i32,
    Focus = 11i32,
    Geometry = 7i32,
    IMGUI = 18i32,
    Keyboard = 6i32,
    Navigation = 14i32,
    Pointer = 1i32,
    PointerDown = 3i32,
    PointerMove = 2i32,
    Style = 8i32,
    StyleTransition = 13i32,
    Tooltip = 16i32,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EventCategory")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::EventCategory {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "EventCategory";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EventCategory")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::UIElements::EventCategory {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EventCategory")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::UIElements::EventCategory {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EventCategory")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::UIElements::EventCategory {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EventCategory")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::UIElements::EventCategory {
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
