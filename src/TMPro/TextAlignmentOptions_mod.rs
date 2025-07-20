#[cfg(feature = "TMPro+TextAlignmentOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlignmentOptions {
    #[default]
    Baseline = 2050i32,
    BaselineFlush = 2064i32,
    BaselineGeoAligned = 2080i32,
    BaselineJustified = 2056i32,
    BaselineLeft = 2049i32,
    BaselineRight = 2052i32,
    Bottom = 1026i32,
    BottomFlush = 1040i32,
    BottomGeoAligned = 1056i32,
    BottomJustified = 1032i32,
    BottomLeft = 1025i32,
    BottomRight = 1028i32,
    Capline = 8194i32,
    CaplineFlush = 8208i32,
    CaplineGeoAligned = 8224i32,
    CaplineJustified = 8200i32,
    CaplineLeft = 8193i32,
    CaplineRight = 8196i32,
    Center = 514i32,
    CenterGeoAligned = 544i32,
    Converted = 65535i32,
    Flush = 528i32,
    Justified = 520i32,
    Left = 513i32,
    Midline = 4098i32,
    MidlineFlush = 4112i32,
    MidlineGeoAligned = 4128i32,
    MidlineJustified = 4104i32,
    MidlineLeft = 4097i32,
    MidlineRight = 4100i32,
    Right = 516i32,
    Top = 258i32,
    TopFlush = 272i32,
    TopGeoAligned = 288i32,
    TopJustified = 264i32,
    TopLeft = 257i32,
    TopRight = 260i32,
}
#[cfg(feature = "TMPro+TextAlignmentOptions")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TextAlignmentOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TextAlignmentOptions";
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
#[cfg(feature = "TMPro+TextAlignmentOptions")]
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::TextAlignmentOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "TMPro+TextAlignmentOptions")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::TMPro::TextAlignmentOptions {
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
#[cfg(feature = "TMPro+TextAlignmentOptions")]
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::TextAlignmentOptions {
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
#[cfg(feature = "TMPro+TextAlignmentOptions")]
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::TextAlignmentOptions {
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
