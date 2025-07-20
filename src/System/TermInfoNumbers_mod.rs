#[cfg(feature = "System+TermInfoNumbers")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TermInfoNumbers {
    #[default]
    BitImageEntwining = 31i32,
    BitImageType = 32i32,
    BufferCapacity = 16i32,
    Buttons = 30i32,
    Columns = 0i32,
    DotHorzSpacing = 18i32,
    DotVertSpacing = 17i32,
    InitTabs = 1i32,
    LabelHeight = 9i32,
    LabelWidth = 10i32,
    Last = 33i32,
    Lines = 2i32,
    LinesOfMemory = 3i32,
    MagicCookieGlitch = 4i32,
    MaxAttributes = 11i32,
    MaxColors = 13i32,
    MaxMicroAddress = 19i32,
    MaxMicroJump = 20i32,
    MaxPairs = 14i32,
    MaximumWindows = 12i32,
    MicroColSize = 21i32,
    MicroLineSize = 22i32,
    NoColorVideo = 15i32,
    NumLabels = 8i32,
    NumberOfPins = 23i32,
    OutputResChar = 24i32,
    OutputResHorzInch = 26i32,
    OutputResLine = 25i32,
    OutputResVertInch = 27i32,
    PaddingBaudRate = 5i32,
    PrintRate = 28i32,
    VirtualTerminal = 6i32,
    WideCharSize = 29i32,
    WidthStatusLine = 7i32,
}
#[cfg(feature = "System+TermInfoNumbers")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::TermInfoNumbers {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "TermInfoNumbers";
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
#[cfg(feature = "System+TermInfoNumbers")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::TermInfoNumbers {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+TermInfoNumbers")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::TermInfoNumbers {
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
#[cfg(feature = "System+TermInfoNumbers")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::TermInfoNumbers {
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
#[cfg(feature = "System+TermInfoNumbers")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::TermInfoNumbers {
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
