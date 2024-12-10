#[cfg(feature = "MS+Internal+Xml+XPath+XPathScanner")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathScanner {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _xpathExpr: *mut quest_hook::libil2cpp::Il2CppString,
    pub _xpathExprIndex: i32,
    pub _kind: crate::MS::Internal::Xml::XPath::XPathScanner_LexKind,
    pub _currentChar: char,
    pub _name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _prefix: *mut quest_hook::libil2cpp::Il2CppString,
    pub _stringValue: *mut quest_hook::libil2cpp::Il2CppString,
    pub _numberValue: f64,
    pub _canBeFunction: bool,
    pub _xmlCharType: crate::System::Xml::XmlCharType,
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathScanner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::XPathScanner =>
    "MS.Internal.Xml.XPath"."XPathScanner"
);
#[cfg(feature = "MS+Internal+Xml+XPath+XPathScanner")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::XPathScanner {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathScanner")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::XPathScanner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathScanner")]
impl crate::MS::Internal::Xml::XPath::XPathScanner {
    #[cfg(feature = "MS+Internal+Xml+XPath+XPathScanner+LexKind")]
    pub type LexKind = crate::MS::Internal::Xml::XPath::XPathScanner_LexKind;
    pub fn New(
        xpathExpr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xpathExpr))?;
        Ok(__cordl_object.into())
    }
    pub fn NextChar(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("NextChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NextLex(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("NextLex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanFraction(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("ScanFraction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ScanName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanNumber(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("ScanNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ScanString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SkipSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SkipSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        xpathExpr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xpathExpr))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanBeFunction(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanBeFunction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("get_CurrentChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Kind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::MS::Internal::Xml::XPath::XPathScanner_LexKind,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::MS::Internal::Xml::XPath::XPathScanner_LexKind = __cordl_object
            .invoke("get_Kind", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NumberValue(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_NumberValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Prefix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SourceText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_SourceText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StringValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_StringValue", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathScanner")]
impl quest_hook::libil2cpp::ObjectType
for crate::MS::Internal::Xml::XPath::XPathScanner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathScanner+LexKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XPathScanner_LexKind {
    And = 65i32,
    Apos = 39i32,
    At = 64i32,
    Axe = 97i32,
    Bang = 33i32,
    Comma = 44i32,
    Dollar = 36i32,
    Dot = 46i32,
    DotDot = 68i32,
    Eof = 69i32,
    Eq = 61i32,
    Ge = 71i32,
    Gt = 62i32,
    LBracket = 91i32,
    LParens = 40i32,
    Le = 76i32,
    Lt = 60i32,
    Minus = 45i32,
    Name = 110i32,
    Ne = 78i32,
    Number = 100i32,
    Or = 79i32,
    Plus = 43i32,
    Quote = 34i32,
    RBracket = 93i32,
    RParens = 41i32,
    Slash = 47i32,
    SlashSlash = 83i32,
    Star = 42i32,
    String = 115i32,
    Union = 124i32,
}
#[cfg(feature = "MS+Internal+Xml+XPath+XPathScanner+LexKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::XPathScanner_LexKind
    => "MS.Internal.Xml.XPath"."XPathScanner/LexKind"
);
