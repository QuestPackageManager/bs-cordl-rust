#[cfg(feature = "cordl_class_System+Xml+XmlNodeType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum XmlNodeType {
    #[default]
    Attribute = 2i32,
    CDATA = 4i32,
    Comment = 8i32,
    Document = 9i32,
    DocumentFragment = 11i32,
    DocumentType = 10i32,
    Element = 1i32,
    EndElement = 15i32,
    EndEntity = 16i32,
    Entity = 6i32,
    EntityReference = 5i32,
    None = 0i32,
    Notation = 12i32,
    ProcessingInstruction = 7i32,
    SignificantWhitespace = 14i32,
    Text = 3i32,
    Whitespace = 13i32,
    XmlDeclaration = 17i32,
}
#[cfg(feature = "cordl_class_System+Xml+XmlNodeType")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::XmlNodeType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "XmlNodeType";
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
#[cfg(feature = "cordl_class_System+Xml+XmlNodeType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Xml::XmlNodeType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Xml+XmlNodeType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Xml::XmlNodeType {
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
#[cfg(feature = "cordl_class_System+Xml+XmlNodeType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Xml::XmlNodeType {
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
#[cfg(feature = "cordl_class_System+Xml+XmlNodeType")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Xml::XmlNodeType {
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
