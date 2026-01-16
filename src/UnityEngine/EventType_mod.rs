#[cfg(feature = "cordl_class_UnityEngine+EventType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum EventType {
    #[default]
    ContextClick = 16i32,
    DragExited = 15i32,
    DragPerform = 10i32,
    DragUpdated = 9i32,
    ExecuteCommand = 14i32,
    Ignore = 11i32,
    KeyDown = 4i32,
    KeyUp = 5i32,
    Layout = 8i32,
    MouseDown = 0i32,
    MouseDrag = 3i32,
    MouseEnterWindow = 20i32,
    MouseLeaveWindow = 21i32,
    MouseMove = 2i32,
    MouseUp = 1i32,
    Repaint = 7i32,
    ScrollWheel = 6i32,
    TouchDown = 30i32,
    TouchEnter = 33i32,
    TouchLeave = 34i32,
    TouchMove = 32i32,
    TouchStationary = 35i32,
    TouchUp = 31i32,
    Used = 12i32,
    ValidateCommand = 13i32,
}
#[cfg(feature = "cordl_class_UnityEngine+EventType")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::EventType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "EventType";
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
#[cfg(feature = "cordl_class_UnityEngine+EventType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::EventType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+EventType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::EventType {
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
#[cfg(feature = "cordl_class_UnityEngine+EventType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::EventType {
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
#[cfg(feature = "cordl_class_UnityEngine+EventType")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::EventType {
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
