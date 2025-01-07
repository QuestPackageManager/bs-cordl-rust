#[cfg(feature = "System+Reflection+BindingFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BindingFlags {
    #[default]
    CreateInstance = 512i32,
    DeclaredOnly = 2i32,
    Default = 0i32,
    DoNotWrapExceptions = 33554432i32,
    ExactBinding = 65536i32,
    FlattenHierarchy = 64i32,
    GetField = 1024i32,
    GetProperty = 4096i32,
    IgnoreCase = 1i32,
    IgnoreReturn = 16777216i32,
    Instance = 4i32,
    InvokeMethod = 256i32,
    NonPublic = 32i32,
    OptionalParamBinding = 262144i32,
    Public = 16i32,
    PutDispProperty = 16384i32,
    PutRefDispProperty = 32768i32,
    SetField = 2048i32,
    SetProperty = 8192i32,
    Static = 8i32,
    SuppressChangeType = 131072i32,
}
#[cfg(feature = "System+Reflection+BindingFlags")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Reflection::BindingFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Reflection";
    const CLASS_NAME: &'static str = "BindingFlags";
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
#[cfg(feature = "System+Reflection+BindingFlags")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Reflection::BindingFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Reflection+BindingFlags")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Reflection::BindingFlags {
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
#[cfg(feature = "System+Reflection+BindingFlags")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Reflection::BindingFlags {
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
#[cfg(feature = "System+Reflection+BindingFlags")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Reflection::BindingFlags {
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
