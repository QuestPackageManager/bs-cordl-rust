#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
#[repr(C)]
#[derive(Debug)]
pub struct Function {
    __cordl_parent: crate::MS::Internal::Xml::XPath::AstNode,
    pub _functionType: crate::MS::Internal::Xml::XPath::Function_FunctionType,
    pub _argumentList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::MS::Internal::Xml::XPath::AstNode,
        >,
    >,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::Function =>
    "MS.Internal.Xml.XPath"."Function"
);
#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::Function {
    type Target = crate::MS::Internal::Xml::XPath::AstNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::Function {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
impl crate::MS::Internal::Xml::XPath::Function {
    #[cfg(feature = "MS+Internal+Xml+XPath+Function+FunctionType")]
    pub type FunctionType = crate::MS::Internal::Xml::XPath::Function_FunctionType;
    pub fn New_Function_FunctionType_AstNode2(
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        arg: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ftype, arg))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Function_FunctionType_List_1_0(
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        argumentList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::MS::Internal::Xml::XPath::AstNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ftype, argumentList))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_List_1_1(
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        argumentList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::MS::Internal::Xml::XPath::AstNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (prefix, name, argumentList))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Function_FunctionType_AstNode2(
        &mut self,
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        arg: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ftype, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Function_FunctionType_List_1_0(
        &mut self,
        ftype: crate::MS::Internal::Xml::XPath::Function_FunctionType,
        argumentList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::MS::Internal::Xml::XPath::AstNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ftype, argumentList))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_List_1_1(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        argumentList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::MS::Internal::Xml::XPath::AstNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (prefix, name, argumentList))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReturnType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XPath::XPathResultType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XPath::XPathResultType = __cordl_object
            .invoke("get_ReturnType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::MS::Internal::Xml::XPath::AstNode_AstType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::MS::Internal::Xml::XPath::AstNode_AstType = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function")]
impl quest_hook::libil2cpp::ObjectType for crate::MS::Internal::Xml::XPath::Function {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function+FunctionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function_FunctionType {
    FuncBoolean = 8i32,
    FuncCeiling = 25i32,
    FuncConcat = 13i32,
    FuncContains = 15i32,
    FuncCount = 2i32,
    FuncFalse = 11i32,
    FuncFloor = 24i32,
    FuncID = 3i32,
    FuncLang = 22i32,
    FuncLast = 0i32,
    FuncLocalName = 4i32,
    FuncName = 6i32,
    FuncNameSpaceUri = 5i32,
    FuncNormalize = 20i32,
    FuncNot = 12i32,
    FuncNumber = 9i32,
    FuncPosition = 1i32,
    FuncRound = 26i32,
    FuncStartsWith = 14i32,
    FuncString = 7i32,
    FuncStringLength = 19i32,
    FuncSubstring = 18i32,
    FuncSubstringAfter = 17i32,
    FuncSubstringBefore = 16i32,
    FuncSum = 23i32,
    FuncTranslate = 21i32,
    FuncTrue = 10i32,
    FuncUserDefined = 27i32,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Function+FunctionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::Function_FunctionType
    => "MS.Internal.Xml.XPath"."Function/FunctionType"
);
