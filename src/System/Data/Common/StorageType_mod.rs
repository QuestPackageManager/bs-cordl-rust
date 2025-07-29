#[cfg(feature = "cordl_class_System+Data+Common+StorageType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StorageType {
    #[default]
    BigInteger = 24i32,
    Boolean = 3i32,
    Byte = 6i32,
    ByteArray = 20i32,
    Char = 4i32,
    CharArray = 21i32,
    DBNull = 2i32,
    DateTime = 16i32,
    DateTimeOffset = 23i32,
    Decimal = 15i32,
    Double = 14i32,
    Empty = 0i32,
    Guid = 19i32,
    Int16 = 7i32,
    Int32 = 9i32,
    Int64 = 11i32,
    Object = 1i32,
    SByte = 5i32,
    Single = 13i32,
    SqlBinary = 26i32,
    SqlBoolean = 27i32,
    SqlByte = 28i32,
    SqlBytes = 29i32,
    SqlChars = 30i32,
    SqlDateTime = 31i32,
    SqlDecimal = 32i32,
    SqlDouble = 33i32,
    SqlGuid = 34i32,
    SqlInt16 = 35i32,
    SqlInt32 = 36i32,
    SqlInt64 = 37i32,
    SqlMoney = 38i32,
    SqlSingle = 39i32,
    SqlString = 40i32,
    String = 18i32,
    TimeSpan = 17i32,
    Type = 22i32,
    UInt16 = 8i32,
    UInt32 = 10i32,
    UInt64 = 12i32,
    Uri = 25i32,
}
#[cfg(feature = "cordl_class_System+Data+Common+StorageType")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::Common::StorageType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Data.Common";
    const CLASS_NAME: &'static str = "StorageType";
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
#[cfg(feature = "cordl_class_System+Data+Common+StorageType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Data::Common::StorageType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Data+Common+StorageType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Data::Common::StorageType {
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
#[cfg(feature = "cordl_class_System+Data+Common+StorageType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Data::Common::StorageType {
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
#[cfg(feature = "cordl_class_System+Data+Common+StorageType")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Data::Common::StorageType {
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
