#[cfg(feature = "System+Runtime+InteropServices+UnmanagedType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnmanagedType {
    #[default]
    AnsiBStr = 35i32,
    AsAny = 40i32,
    BStr = 19i32,
    Bool = 2i32,
    ByValArray = 30i32,
    ByValTStr = 23i32,
    Currency = 15i32,
    CustomMarshaler = 44i32,
    Error = 45i32,
    FunctionPtr = 38i32,
    HString = 47i32,
    I1 = 3i32,
    I2 = 5i32,
    I4 = 7i32,
    I8 = 9i32,
    IDispatch = 26i32,
    IInspectable = 46i32,
    IUnknown = 25i32,
    Interface = 28i32,
    LPArray = 42i32,
    LPStr = 20i32,
    LPStruct = 43i32,
    LPTStr = 22i32,
    LPUTF8Str = 48i32,
    LPWStr = 21i32,
    R4 = 11i32,
    R8 = 12i32,
    SafeArray = 29i32,
    Struct = 27i32,
    SysInt = 31i32,
    SysUInt = 32i32,
    TBStr = 36i32,
    U1 = 4i32,
    U2 = 6i32,
    U4 = 8i32,
    U8 = 10i32,
    VBByRefStr = 34i32,
    VariantBool = 37i32,
}
#[cfg(feature = "System+Runtime+InteropServices+UnmanagedType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::UnmanagedType
    => "System.Runtime.InteropServices"."UnmanagedType"
);
