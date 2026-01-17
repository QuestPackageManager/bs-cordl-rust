#[cfg(feature = "cordl_class_UnityEngine+Rendering+BlendOp")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum BlendOp {
    #[cfg_attr(feature = "derive_Default", default)]
    Add = 0i32,
    ColorBurn = 27i32,
    ColorDodge = 26i32,
    Darken = 24i32,
    Difference = 30i32,
    Exclusion = 31i32,
    HSLColor = 34i32,
    HSLHue = 32i32,
    HSLLuminosity = 35i32,
    HSLSaturation = 33i32,
    HardLight = 28i32,
    Lighten = 25i32,
    LogicalAnd = 11i32,
    LogicalAndInverted = 18i32,
    LogicalAndReverse = 17i32,
    LogicalClear = 5i32,
    LogicalCopy = 7i32,
    LogicalCopyInverted = 8i32,
    LogicalEquivalence = 16i32,
    LogicalInvert = 10i32,
    LogicalNand = 12i32,
    LogicalNoop = 9i32,
    LogicalNor = 14i32,
    LogicalOr = 13i32,
    LogicalOrInverted = 20i32,
    LogicalOrReverse = 19i32,
    LogicalSet = 6i32,
    LogicalXor = 15i32,
    Max = 4i32,
    Min = 3i32,
    Multiply = 21i32,
    Overlay = 23i32,
    ReverseSubtract = 2i32,
    Screen = 22i32,
    SoftLight = 29i32,
    Subtract = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BlendOp")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::BlendOp {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "BlendOp";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BlendOp")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Rendering::BlendOp {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BlendOp")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Rendering::BlendOp {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BlendOp")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Rendering::BlendOp {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BlendOp")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Rendering::BlendOp {
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
