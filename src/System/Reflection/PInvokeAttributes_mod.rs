#[cfg(feature = "cordl_class_System+Reflection+PInvokeAttributes")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum PInvokeAttributes {
    #[cfg_attr(feature = "derive_Default", default)]
    BestFitDisabled = 32i32,
    BestFitEnabled = 16i32,
    BestFitMask = 48i32,
    BestFitUseAssem = 0i32,
    CallConvCdecl = 512i32,
    CallConvFastcall = 1280i32,
    CallConvMask = 1792i32,
    CallConvStdcall = 768i32,
    CallConvThiscall = 1024i32,
    CallConvWinapi = 256i32,
    CharSetAnsi = 2i32,
    CharSetAuto = 6i32,
    CharSetUnicode = 4i32,
    MaxValue = 65535i32,
    NoMangle = 1i32,
    SupportsLastError = 64i32,
    ThrowOnUnmappableCharDisabled = 8192i32,
    ThrowOnUnmappableCharEnabled = 4096i32,
    ThrowOnUnmappableCharMask = 12288i32,
}
#[cfg(feature = "cordl_class_System+Reflection+PInvokeAttributes")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Reflection::PInvokeAttributes {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Reflection";
    const CLASS_NAME: &'static str = "PInvokeAttributes";
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
#[cfg(feature = "cordl_class_System+Reflection+PInvokeAttributes")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Reflection::PInvokeAttributes {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Reflection+PInvokeAttributes")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Reflection::PInvokeAttributes {
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
#[cfg(feature = "cordl_class_System+Reflection+PInvokeAttributes")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Reflection::PInvokeAttributes {
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
#[cfg(feature = "cordl_class_System+Reflection+PInvokeAttributes")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Reflection::PInvokeAttributes {
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
