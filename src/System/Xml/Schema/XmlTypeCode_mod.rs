#[cfg(feature = "System+Xml+Schema+XmlTypeCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XmlTypeCode {
    #[default]
    AnyAtomicType = 10i32,
    AnyUri = 28i32,
    Attribute = 5i32,
    Base64Binary = 27i32,
    Boolean = 13i32,
    Byte = 46i32,
    Comment = 8i32,
    Date = 20i32,
    DateTime = 18i32,
    DayTimeDuration = 54i32,
    Decimal = 14i32,
    Document = 3i32,
    Double = 16i32,
    Duration = 17i32,
    Element = 4i32,
    Entity = 39i32,
    Float = 15i32,
    GDay = 24i32,
    GMonth = 25i32,
    GMonthDay = 23i32,
    GYear = 22i32,
    GYearMonth = 21i32,
    HexBinary = 26i32,
    Id = 37i32,
    Idref = 38i32,
    Int = 44i32,
    Integer = 40i32,
    Item = 1i32,
    Language = 33i32,
    Long = 43i32,
    NCName = 36i32,
    Name = 35i32,
    Namespace = 6i32,
    NegativeInteger = 42i32,
    NmToken = 34i32,
    Node = 2i32,
    NonNegativeInteger = 47i32,
    NonPositiveInteger = 41i32,
    None = 0i32,
    NormalizedString = 31i32,
    Notation = 30i32,
    PositiveInteger = 52i32,
    ProcessingInstruction = 7i32,
    QName = 29i32,
    Short = 45i32,
    String = 12i32,
    Text = 9i32,
    Time = 19i32,
    Token = 32i32,
    UnsignedByte = 51i32,
    UnsignedInt = 49i32,
    UnsignedLong = 48i32,
    UnsignedShort = 50i32,
    UntypedAtomic = 11i32,
    YearMonthDuration = 53i32,
}
#[cfg(feature = "System+Xml+Schema+XmlTypeCode")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::XmlTypeCode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "XmlTypeCode";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Xml::Schema::XmlTypeCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::XmlTypeCode {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Xml::Schema::XmlTypeCode {
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
unsafe impl quest_hook::libil2cpp::Return for crate::System::Xml::Schema::XmlTypeCode {
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
