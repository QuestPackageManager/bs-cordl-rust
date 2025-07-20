#[cfg(feature = "BeatmapSaveDataCommon+BeatmapEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BeatmapEventType {
    #[default]
    BpmChange = 100i32,
    ColorBoostEventType = 5i32,
    EarlyRotationEventType = 14i32,
    Event0 = 0i32,
    Event1 = 1i32,
    Event10 = 10i32,
    Event11 = 11i32,
    Event12 = 12i32,
    Event13 = 13i32,
    Event15 = 15i32,
    Event16 = 16i32,
    Event17 = 17i32,
    Event2 = 2i32,
    Event3 = 3i32,
    Event4 = 4i32,
    Event6 = 6i32,
    Event7 = 7i32,
    Event8 = 8i32,
    Event9 = 9i32,
    Special0 = 40i32,
    Special1 = 41i32,
    Special2 = 42i32,
    Special3 = 43i32,
    VoidEvent = -1i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+BeatmapEventType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataCommon::BeatmapEventType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataCommon";
    const CLASS_NAME: &'static str = "BeatmapEventType";
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
#[cfg(feature = "BeatmapSaveDataCommon+BeatmapEventType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatmapSaveDataCommon::BeatmapEventType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+BeatmapEventType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatmapSaveDataCommon::BeatmapEventType {
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
#[cfg(feature = "BeatmapSaveDataCommon+BeatmapEventType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatmapSaveDataCommon::BeatmapEventType {
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
#[cfg(feature = "BeatmapSaveDataCommon+BeatmapEventType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatmapSaveDataCommon::BeatmapEventType {
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
