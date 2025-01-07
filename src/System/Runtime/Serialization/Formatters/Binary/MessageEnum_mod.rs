#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MessageEnum")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageEnum {
    #[default]
    ArgsInArray = 8i32,
    ArgsInline = 2i32,
    ArgsIsArray = 4i32,
    ContextInArray = 64i32,
    ContextInline = 32i32,
    ExceptionInArray = 8192i32,
    GenericMethod = 32768i32,
    MethodSignatureInArray = 128i32,
    NoArgs = 1i32,
    NoContext = 16i32,
    NoReturnValue = 512i32,
    PropertyInArray = 256i32,
    ReturnValueInArray = 4096i32,
    ReturnValueInline = 2048i32,
    ReturnValueVoid = 1024i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MessageEnum")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::Formatters::Binary::MessageEnum {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization.Formatters.Binary";
    const CLASS_NAME: &'static str = "MessageEnum";
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MessageEnum")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Runtime::Serialization::Formatters::Binary::MessageEnum {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MessageEnum")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Runtime::Serialization::Formatters::Binary::MessageEnum {
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MessageEnum")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Runtime::Serialization::Formatters::Binary::MessageEnum {
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+MessageEnum")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Runtime::Serialization::Formatters::Binary::MessageEnum {
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
