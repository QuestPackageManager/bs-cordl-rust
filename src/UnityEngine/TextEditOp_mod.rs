#[cfg(feature = "UnityEngine+TextEditOp")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextEditOp {
    #[default]
    Backspace = 19i32,
    Cut = 23i32,
    Delete = 18i32,
    DeleteLineBack = 22i32,
    DeleteWordBack = 20i32,
    DeleteWordForward = 21i32,
    MoveDown = 3i32,
    MoveGraphicalLineEnd = 11i32,
    MoveGraphicalLineStart = 10i32,
    MoveLeft = 0i32,
    MoveLineEnd = 5i32,
    MoveLineStart = 4i32,
    MovePageDown = 9i32,
    MovePageUp = 8i32,
    MoveParagraphBackward = 15i32,
    MoveParagraphForward = 14i32,
    MoveRight = 1i32,
    MoveTextEnd = 7i32,
    MoveTextStart = 6i32,
    MoveToEndOfPreviousWord = 17i32,
    MoveToStartOfNextWord = 16i32,
    MoveUp = 2i32,
    MoveWordLeft = 12i32,
    MoveWordRight = 13i32,
    Paste = 24i32,
    ScrollEnd = 26i32,
    ScrollPageDown = 28i32,
    ScrollPageUp = 27i32,
    ScrollStart = 25i32,
}
#[cfg(feature = "UnityEngine+TextEditOp")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::TextEditOp {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "TextEditOp";
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
#[cfg(feature = "UnityEngine+TextEditOp")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::TextEditOp {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+TextEditOp")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::TextEditOp {
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
#[cfg(feature = "UnityEngine+TextEditOp")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::TextEditOp {
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
#[cfg(feature = "UnityEngine+TextEditOp")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::TextEditOp {
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
