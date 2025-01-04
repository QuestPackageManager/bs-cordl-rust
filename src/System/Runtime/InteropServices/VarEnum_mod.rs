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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::VarEnum =>
    "System.Runtime.InteropServices"."VarEnum"
);
