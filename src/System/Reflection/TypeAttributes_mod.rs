#[cfg(feature = "System+Reflection+TypeAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeAttributes {
    #[default]
    Abstract = 128i32,
    AnsiClass = 0i32,
    AutoClass = 131072i32,
    BeforeFieldInit = 1048576i32,
    ClassSemanticsMask = 32i32,
    CustomFormatClass = 196608i32,
    CustomFormatMask = 12582912i32,
    ExplicitLayout = 16i32,
    HasSecurity = 262144i32,
    Import = 4096i32,
    LayoutMask = 24i32,
    NestedAssembly = 5i32,
    NestedFamANDAssem = 6i32,
    NestedFamORAssem = 7i32,
    NestedFamily = 4i32,
    NestedPrivate = 3i32,
    NestedPublic = 2i32,
    Public = 1i32,
    RTSpecialName = 2048i32,
    ReservedMask = 264192i32,
    Sealed = 256i32,
    SequentialLayout = 8i32,
    Serializable = 8192i32,
    SpecialName = 1024i32,
    UnicodeClass = 65536i32,
    WindowsRuntime = 16384i32,
}
#[cfg(feature = "System+Reflection+TypeAttributes")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Reflection::TypeAttributes {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Reflection";
    const CLASS_NAME: &'static str = "TypeAttributes";
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
#[cfg(feature = "System+Reflection+TypeAttributes")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Reflection::TypeAttributes {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Reflection+TypeAttributes")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Reflection::TypeAttributes {
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
#[cfg(feature = "System+Reflection+TypeAttributes")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Reflection::TypeAttributes {
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
#[cfg(feature = "System+Reflection+TypeAttributes")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Reflection::TypeAttributes {
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
