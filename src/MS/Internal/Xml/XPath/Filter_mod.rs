#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
#[repr(C)]
#[derive(Debug)]
pub struct Filter {
    __cordl_parent: crate::MS::Internal::Xml::XPath::AstNode,
    pub _input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    pub _condition: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
}
#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MS::Internal::Xml::XPath::Filter =>
    "MS.Internal.Xml.XPath"."Filter"
);
#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
impl std::ops::Deref for crate::MS::Internal::Xml::XPath::Filter {
    type Target = crate::MS::Internal::Xml::XPath::AstNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
impl std::ops::DerefMut for crate::MS::Internal::Xml::XPath::Filter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
impl crate::MS::Internal::Xml::XPath::Filter {
    pub fn New(
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        condition: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, condition))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
        condition: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::AstNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input, condition))?;
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
#[cfg(feature = "MS+Internal+Xml+XPath+Filter")]
impl quest_hook::libil2cpp::ObjectType for crate::MS::Internal::Xml::XPath::Filter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
