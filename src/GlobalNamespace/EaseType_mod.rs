#[cfg(feature = "EaseType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EaseType {
    #[default]
    BeatSaberInOutBack = 32i32,
    BeatSaberInOutBounce = 34i32,
    BeatSaberInOutElastic = 33i32,
    InBack = 23i32,
    InBounce = 29i32,
    InCirc = 20i32,
    InCubic = 8i32,
    InElastic = 26i32,
    InExpo = 17i32,
    InOutBack = 25i32,
    InOutBounce = 31i32,
    InOutCirc = 22i32,
    InOutCubic = 10i32,
    InOutElastic = 28i32,
    InOutExpo = 19i32,
    InOutQuad = 7i32,
    InOutQuart = 13i32,
    InOutQuint = 16i32,
    InOutSine = 4i32,
    InQuad = 5i32,
    InQuart = 11i32,
    InQuint = 14i32,
    InSine = 2i32,
    Linear = 1i32,
    None = 0i32,
    OutBack = 24i32,
    OutBounce = 30i32,
    OutCirc = 21i32,
    OutCubic = 9i32,
    OutElastic = 27i32,
    OutExpo = 18i32,
    OutQuad = 6i32,
    OutQuart = 12i32,
    OutQuint = 15i32,
    OutSine = 3i32,
}
#[cfg(feature = "EaseType")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::EaseType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EaseType";
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
#[cfg(feature = "EaseType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::EaseType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "EaseType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::EaseType {
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
#[cfg(feature = "EaseType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::EaseType {
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
#[cfg(feature = "EaseType")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::EaseType {
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
