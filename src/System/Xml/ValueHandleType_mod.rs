#[cfg(feature = "cordl_class_System+Xml+ValueHandleType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ValueHandleType {
    #[default]
    Base64 = 19i32,
    Char = 22i32,
    ConstString = 25i32,
    DateTime = 13i32,
    Decimal = 12i32,
    Dictionary = 20i32,
    Double = 11i32,
    Empty = 0i32,
    EscapedUTF8 = 18i32,
    False = 2i32,
    Guid = 15i32,
    Int16 = 6i32,
    Int32 = 7i32,
    Int64 = 8i32,
    Int8 = 5i32,
    List = 21i32,
    One = 4i32,
    QName = 24i32,
    Single = 10i32,
    TimeSpan = 14i32,
    True = 1i32,
    UInt64 = 9i32,
    UTF8 = 17i32,
    Unicode = 23i32,
    UniqueId = 16i32,
    Zero = 3i32,
}
#[cfg(feature = "cordl_class_System+Xml+ValueHandleType")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::ValueHandleType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "ValueHandleType";
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
#[cfg(feature = "cordl_class_System+Xml+ValueHandleType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Xml::ValueHandleType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Xml+ValueHandleType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Xml::ValueHandleType {
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
#[cfg(feature = "cordl_class_System+Xml+ValueHandleType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Xml::ValueHandleType {
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
#[cfg(feature = "cordl_class_System+Xml+ValueHandleType")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Xml::ValueHandleType {
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
