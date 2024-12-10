#[cfg(feature = "MS+Internal+Xml+XPath+Operator")]
#[repr(C)]
#[derive(Debug)]
pub struct Operator {
    __cordl_parent: crate::MS::Internal::Xml::XPath::AstNode,
    pub _opType: crate::MS::Internal::Xml::XPath::Operator_Op,
    pub _opnd1: *mut crate::MS::Internal::Xml::XPath::AstNode,
    pub _opnd2: *mut crate::MS::Internal::Xml::XPath::AstNode,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Operator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::Operator =>
    "MS.Internal.Xml.XPath"."Operator"
);
#[cfg(feature = "MS+Internal+Xml+XPath+Operator")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::Operator {
    type Target = crate::MS::Internal::Xml::XPath::AstNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Operator")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::Operator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Operator")]
impl crate::MS::Internal::Xml::XPath::Operator {
    #[cfg(feature = "MS+Internal+Xml+XPath+Operator+Op")]
    pub type Op = crate::MS::Internal::Xml::XPath::Operator_Op;
    pub fn New(
        op: crate::MS::Internal::Xml::XPath::Operator_Op,
        opnd1: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        opnd2: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (op, opnd1, opnd2))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        op: crate::MS::Internal::Xml::XPath::Operator_Op,
        opnd1: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        opnd2: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (op, opnd1, opnd2))?;
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
#[cfg(feature = "MS+Internal+Xml+XPath+Operator")]
impl quest_hook::libil2cpp::ObjectType for crate::MS::Internal::Xml::XPath::Operator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Operator+Op")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator_Op {
    AND = 2i32,
    DIV = 12i32,
    EQ = 3i32,
    GE = 8i32,
    GT = 7i32,
    INVALID = 0i32,
    LE = 6i32,
    LT = 5i32,
    MINUS = 10i32,
    _cordl_MOD = 13i32,
    MUL = 11i32,
    NE = 4i32,
    OR = 1i32,
    PLUS = 9i32,
    UNION = 14i32,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Operator+Op")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::Operator_Op =>
    "MS.Internal.Xml.XPath"."Operator/Op"
);
