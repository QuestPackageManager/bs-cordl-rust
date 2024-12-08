#[cfg(feature = "MS+Internal+Xml+XPath+AstNode")]
#[repr(C)]
#[derive(Debug)]
pub struct AstNode {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "MS+Internal+Xml+XPath+AstNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::AstNode =>
    "MS.Internal.Xml.XPath"."AstNode"
);
#[cfg(feature = "MS+Internal+Xml+XPath+AstNode")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::AstNode {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+AstNode")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::AstNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+AstNode")]
impl crate::MS::Internal::Xml::XPath::AstNode {
    #[cfg(feature = "MS+Internal+Xml+XPath+AstNode+AstType")]
    pub type AstType = crate::MS::Internal::Xml::XPath::AstNode_AstType;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReturnType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XPath::XPathResultType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XPath::XPathResultType = __cordl_object
            .invoke("get_ReturnType", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+AstNode")]
impl quest_hook::libil2cpp::ObjectType for crate::MS::Internal::Xml::XPath::AstNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+AstNode+AstType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstNode_AstType {
    Axis = 0i32,
    ConstantOperand = 3i32,
    Error = 8i32,
    Filter = 2i32,
    Function = 4i32,
    Group = 5i32,
    Operator = 1i32,
    Root = 6i32,
    Variable = 7i32,
}
#[cfg(feature = "MS+Internal+Xml+XPath+AstNode+AstType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::AstNode_AstType =>
    "MS.Internal.Xml.XPath"."AstNode/AstType"
);
