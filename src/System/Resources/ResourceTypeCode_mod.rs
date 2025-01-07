#[cfg(feature = "System+Resources+ResourceTypeCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResourceTypeCode {
    #[default]
    Boolean = 2i32,
    Byte = 4i32,
    ByteArray = 32i32,
    Char = 3i32,
    DateTime = 15i32,
    Decimal = 14i32,
    Double = 13i32,
    Int16 = 6i32,
    Int32 = 8i32,
    Int64 = 10i32,
    LastPrimitive = 16i32,
    Null = 0i32,
    SByte = 5i32,
    Single = 12i32,
    StartOfUserTypes = 64i32,
    Stream = 33i32,
    String = 1i32,
    UInt16 = 7i32,
    UInt32 = 9i32,
    UInt64 = 11i32,
}
#[cfg(feature = "System+Resources+ResourceTypeCode")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Resources::ResourceTypeCode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Resources";
    const CLASS_NAME: &'static str = "ResourceTypeCode";
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
#[cfg(feature = "System+Resources+ResourceTypeCode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Resources::ResourceTypeCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Resources+ResourceTypeCode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Resources::ResourceTypeCode {
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
#[cfg(feature = "System+Resources+ResourceTypeCode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Resources::ResourceTypeCode {
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
#[cfg(feature = "System+Resources+ResourceTypeCode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Resources::ResourceTypeCode {
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
