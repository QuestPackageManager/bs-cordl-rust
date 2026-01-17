#[cfg(feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+BinaryHeaderEnum")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum BinaryHeaderEnum {
    #[cfg_attr(feature = "derive_Default", default)]
    Array = 7i32,
    ArraySingleObject = 16i32,
    ArraySinglePrimitive = 15i32,
    ArraySingleString = 17i32,
    Assembly = 12i32,
    CrossAppDomainAssembly = 20i32,
    CrossAppDomainMap = 18i32,
    CrossAppDomainString = 19i32,
    MemberPrimitiveTyped = 8i32,
    MemberReference = 9i32,
    MessageEnd = 11i32,
    MethodCall = 21i32,
    MethodReturn = 22i32,
    Object = 1i32,
    ObjectNull = 10i32,
    ObjectNullMultiple = 14i32,
    ObjectNullMultiple256 = 13i32,
    ObjectString = 6i32,
    ObjectWithMap = 2i32,
    ObjectWithMapAssemId = 3i32,
    ObjectWithMapTyped = 4i32,
    ObjectWithMapTypedAssemId = 5i32,
    SerializedStreamHeader = 0i32,
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+BinaryHeaderEnum")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization.Formatters.Binary";
    const CLASS_NAME: &'static str = "BinaryHeaderEnum";
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
#[cfg(feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+BinaryHeaderEnum")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+BinaryHeaderEnum")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum
{
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
#[cfg(feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+BinaryHeaderEnum")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum
{
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
#[cfg(feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+BinaryHeaderEnum")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::System::Runtime::Serialization::Formatters::Binary::BinaryHeaderEnum
{
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
