#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+InternalPrimitiveTypeE"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InternalPrimitiveTypeE {
    #[default]
    Boolean = 1i32,
    Byte = 2i32,
    Char = 3i32,
    Currency = 4i32,
    DateTime = 13i32,
    Decimal = 5i32,
    Double = 6i32,
    Int16 = 7i32,
    Int32 = 8i32,
    Int64 = 9i32,
    Invalid = 0i32,
    Null = 17i32,
    SByte = 10i32,
    Single = 11i32,
    String = 18i32,
    TimeSpan = 12i32,
    UInt16 = 14i32,
    UInt32 = 15i32,
    UInt64 = 16i32,
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+InternalPrimitiveTypeE"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization.Formatters.Binary";
    const CLASS_NAME: &'static str = "InternalPrimitiveTypeE";
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
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+InternalPrimitiveTypeE"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+InternalPrimitiveTypeE"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE {
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
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+InternalPrimitiveTypeE"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE {
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
#[cfg(
    feature = "cordl_class_System+Runtime+Serialization+Formatters+Binary+InternalPrimitiveTypeE"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE {
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
