#[cfg(feature = "BeatmapSaveDataCommon+NoteCutDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteCutDirection {
    #[default]
    Any = 8i32,
    Down = 1i32,
    DownLeft = 6i32,
    DownRight = 7i32,
    Left = 2i32,
    None = 9i32,
    Right = 3i32,
    Up = 0i32,
    UpLeft = 4i32,
    UpRight = 5i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+NoteCutDirection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataCommon::NoteCutDirection {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataCommon";
    const CLASS_NAME: &'static str = "NoteCutDirection";
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
#[cfg(feature = "BeatmapSaveDataCommon+NoteCutDirection")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatmapSaveDataCommon::NoteCutDirection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+NoteCutDirection")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatmapSaveDataCommon::NoteCutDirection {
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
#[cfg(feature = "BeatmapSaveDataCommon+NoteCutDirection")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatmapSaveDataCommon::NoteCutDirection {
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
#[cfg(feature = "BeatmapSaveDataCommon+NoteCutDirection")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatmapSaveDataCommon::NoteCutDirection {
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
