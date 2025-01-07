#[cfg(feature = "System+Runtime+InteropServices+VarEnum")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VarEnum {
    #[default]
    VT_ARRAY = 8192i32,
    VT_BLOB = 65i32,
    VT_BLOB_OBJECT = 70i32,
    VT_BOOL = 11i32,
    VT_BSTR = 8i32,
    VT_BYREF = 16384i32,
    VT_CARRAY = 28i32,
    VT_CF = 71i32,
    VT_CLSID = 72i32,
    VT_CY = 6i32,
    VT_DATE = 7i32,
    VT_DECIMAL = 14i32,
    VT_DISPATCH = 9i32,
    VT_EMPTY = 0i32,
    VT_ERROR = 10i32,
    VT_FILETIME = 64i32,
    VT_HRESULT = 25i32,
    VT_I1 = 16i32,
    VT_I2 = 2i32,
    VT_I4 = 3i32,
    VT_I8 = 20i32,
    VT_INT = 22i32,
    VT_LPSTR = 30i32,
    VT_LPWSTR = 31i32,
    VT_NULL = 1i32,
    VT_PTR = 26i32,
    VT_R4 = 4i32,
    VT_R8 = 5i32,
    VT_RECORD = 36i32,
    VT_SAFEARRAY = 27i32,
    VT_STORAGE = 67i32,
    VT_STORED_OBJECT = 69i32,
    VT_STREAM = 66i32,
    VT_STREAMED_OBJECT = 68i32,
    VT_UI1 = 17i32,
    VT_UI2 = 18i32,
    VT_UI4 = 19i32,
    VT_UI8 = 21i32,
    VT_UINT = 23i32,
    VT_UNKNOWN = 13i32,
    VT_USERDEFINED = 29i32,
    VT_VARIANT = 12i32,
    VT_VECTOR = 4096i32,
    VT_VOID = 24i32,
}
#[cfg(feature = "System+Runtime+InteropServices+VarEnum")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::VarEnum {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "VarEnum";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Runtime::InteropServices::VarEnum {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Runtime::InteropServices::VarEnum {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Runtime::InteropServices::VarEnum {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Runtime::InteropServices::VarEnum {
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
