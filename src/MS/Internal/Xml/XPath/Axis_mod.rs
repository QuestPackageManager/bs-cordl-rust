#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
#[repr(C)]
#[derive(Debug)]
pub struct Axis {
    __cordl_parent: crate::MS::Internal::Xml::XPath::AstNode,
    pub _axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
    pub _input: *mut crate::MS::Internal::Xml::XPath::AstNode,
    pub _prefix: *mut quest_hook::libil2cpp::Il2CppString,
    pub _name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _nodeType: crate::System::Xml::XPath::XPathNodeType,
    pub abbrAxis: bool,
    pub _urn: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::Axis =>
    "MS.Internal.Xml.XPath"."Axis"
);
#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::Axis {
    type Target = crate::MS::Internal::Xml::XPath::AstNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::Axis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
impl crate::MS::Internal::Xml::XPath::Axis {
    #[cfg(feature = "MS+Internal+Xml+XPath+Axis+AxisType")]
    pub type AxisType = crate::MS::Internal::Xml::XPath::Axis_AxisType;
    pub fn New_Axis_AxisType_AstNode1(
        axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axisType, input))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_XPathNodeType0(
        axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nodetype: crate::System::Xml::XPath::XPathNodeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axisType, input, prefix, name, nodetype))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Axis_AxisType_AstNode1(
        &mut self,
        axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (axisType, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_XPathNodeType0(
        &mut self,
        axisType: crate::MS::Internal::Xml::XPath::Axis_AxisType,
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nodetype: crate::System::Xml::XPath::XPathNodeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (axisType, input, prefix, name, nodetype))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AbbrAxis(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AbbrAxis", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Input(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::AstNode,
        > = __cordl_object.invoke("get_Input", ())?;
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
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XPath::XPathNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XPath::XPathNodeType = __cordl_object
            .invoke("get_NodeType", ())?;
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
    pub fn get_TypeOfAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::MS::Internal::Xml::XPath::Axis_AxisType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::MS::Internal::Xml::XPath::Axis_AxisType = __cordl_object
            .invoke("get_TypeOfAxis", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Urn(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Urn", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Input(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Input", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Urn(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Urn", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis")]
impl quest_hook::libil2cpp::ObjectType for crate::MS::Internal::Xml::XPath::Axis {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis+AxisType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis_AxisType {
    Ancestor = 0i32,
    AncestorOrSelf = 1i32,
    Attribute = 2i32,
    Child = 3i32,
    Descendant = 4i32,
    DescendantOrSelf = 5i32,
    Following = 6i32,
    FollowingSibling = 7i32,
    Namespace = 8i32,
    None = 13i32,
    Parent = 9i32,
    Preceding = 10i32,
    PrecedingSibling = 11i32,
    _cordl_Self = 12i32,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Axis+AxisType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::Axis_AxisType =>
    "MS.Internal.Xml.XPath"."Axis/AxisType"
);
