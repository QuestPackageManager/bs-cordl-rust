#[cfg(feature = "cordl_class_System+Runtime+InteropServices+UnmanagedType")]
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
#[cfg(feature = "cordl_class_System+Runtime+InteropServices+UnmanagedType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::UnmanagedType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "UnmanagedType";
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
#[cfg(feature = "cordl_class_System+Runtime+InteropServices+UnmanagedType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Runtime::InteropServices::UnmanagedType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Runtime+InteropServices+UnmanagedType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Runtime::InteropServices::UnmanagedType {
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
#[cfg(feature = "cordl_class_System+Runtime+InteropServices+UnmanagedType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Runtime::InteropServices::UnmanagedType {
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
#[cfg(feature = "cordl_class_System+Runtime+InteropServices+UnmanagedType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Runtime::InteropServices::UnmanagedType {
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
