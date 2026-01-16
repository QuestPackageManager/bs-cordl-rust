#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+PrimitiveTypeCode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum PrimitiveTypeCode {
    #[default]
    BigInteger = 36i32,
    BigIntegerNullable = 37i32,
    Boolean = 4i32,
    BooleanNullable = 5i32,
    Byte = 14i32,
    ByteNullable = 15i32,
    Bytes = 40i32,
    Char = 2i32,
    CharNullable = 3i32,
    DBNull = 41i32,
    DateTime = 26i32,
    DateTimeNullable = 27i32,
    DateTimeOffset = 28i32,
    DateTimeOffsetNullable = 29i32,
    Decimal = 30i32,
    DecimalNullable = 31i32,
    Double = 24i32,
    DoubleNullable = 25i32,
    Empty = 0i32,
    Guid = 32i32,
    GuidNullable = 33i32,
    Int16 = 8i32,
    Int16Nullable = 9i32,
    Int32 = 12i32,
    Int32Nullable = 13i32,
    Int64 = 18i32,
    Int64Nullable = 19i32,
    Object = 1i32,
    SByte = 6i32,
    SByteNullable = 7i32,
    Single = 22i32,
    SingleNullable = 23i32,
    String = 39i32,
    TimeSpan = 34i32,
    TimeSpanNullable = 35i32,
    UInt16 = 10i32,
    UInt16Nullable = 11i32,
    UInt32 = 16i32,
    UInt32Nullable = 17i32,
    UInt64 = 20i32,
    UInt64Nullable = 21i32,
    Uri = 38i32,
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+PrimitiveTypeCode")]
unsafe impl quest_hook::libil2cpp::Type for crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "PrimitiveTypeCode";
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
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+PrimitiveTypeCode")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+PrimitiveTypeCode")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode
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
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+PrimitiveTypeCode")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode
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
#[cfg(feature = "cordl_class_Newtonsoft+Json+Utilities+PrimitiveTypeCode")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode
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
