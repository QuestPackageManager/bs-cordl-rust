#[cfg(feature = "BeatmapSaveDataCommon+EaseType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EaseType {
    #[default]
    BeatSaberInOutBack = 100i32,
    BeatSaberInOutBounce = 102i32,
    BeatSaberInOutElastic = 101i32,
    InBack = 22i32,
    InBounce = 28i32,
    InCirc = 19i32,
    InCubic = 7i32,
    InElastic = 25i32,
    InExpo = 16i32,
    InOutBack = 24i32,
    InOutBounce = 30i32,
    InOutCirc = 21i32,
    InOutCubic = 9i32,
    InOutElastic = 27i32,
    InOutExpo = 18i32,
    InOutQuad = 3i32,
    InOutQuart = 12i32,
    InOutQuint = 15i32,
    InOutSine = 6i32,
    InQuad = 1i32,
    InQuart = 10i32,
    InQuint = 13i32,
    InSine = 4i32,
    Linear = 0i32,
    None = -1i32,
    OutBack = 23i32,
    OutBounce = 29i32,
    OutCirc = 20i32,
    OutCubic = 8i32,
    OutElastic = 26i32,
    OutExpo = 17i32,
    OutQuad = 2i32,
    OutQuart = 11i32,
    OutQuint = 14i32,
    OutSine = 5i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+EaseType")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatmapSaveDataCommon::EaseType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataCommon";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::BeatmapSaveDataCommon::EaseType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::BeatmapSaveDataCommon::EaseType {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::BeatmapSaveDataCommon::EaseType {
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
unsafe impl quest_hook::libil2cpp::Return for crate::BeatmapSaveDataCommon::EaseType {
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
