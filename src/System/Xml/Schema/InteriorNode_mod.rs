#[cfg(feature = "System+Xml+Schema+InteriorNode")]
#[repr(C)]
#[derive(Debug)]
pub struct InteriorNode {
    __cordl_parent: crate::System::Xml::Schema::SyntaxTreeNode,
    pub leftChild: *mut crate::System::Xml::Schema::SyntaxTreeNode,
    pub rightChild: *mut crate::System::Xml::Schema::SyntaxTreeNode,
}
#[cfg(feature = "System+Xml+Schema+InteriorNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::InteriorNode =>
    "System.Xml.Schema"."InteriorNode"
);
#[cfg(feature = "System+Xml+Schema+InteriorNode")]
impl std::ops::Deref for crate::System::Xml::Schema::InteriorNode {
    type Target = crate::System::Xml::Schema::SyntaxTreeNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+InteriorNode")]
impl std::ops::DerefMut for crate::System::Xml::Schema::InteriorNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+InteriorNode")]
impl crate::System::Xml::Schema::InteriorNode {
    pub fn get_LeftChild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SyntaxTreeNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SyntaxTreeNode = __cordl_object
            .invoke("get_LeftChild", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_LeftChild(
        &mut self,
        value: *mut crate::System::Xml::Schema::SyntaxTreeNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LeftChild", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_RightChild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SyntaxTreeNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SyntaxTreeNode = __cordl_object
            .invoke("get_RightChild", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExpandTreeNoRecursive(
        &mut self,
        parent: *mut crate::System::Xml::Schema::InteriorNode,
        symbols: *mut crate::System::Xml::Schema::SymbolsDictionary,
        positions: *mut crate::System::Xml::Schema::Positions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandTreeNoRecursive", (parent, symbols, positions))?;
        Ok(__cordl_ret)
    }
    pub fn ExpandTree(
        &mut self,
        parent: *mut crate::System::Xml::Schema::InteriorNode,
        symbols: *mut crate::System::Xml::Schema::SymbolsDictionary,
        positions: *mut crate::System::Xml::Schema::Positions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandTree", (parent, symbols, positions))?;
        Ok(__cordl_ret)
    }
    pub fn set_RightChild(
        &mut self,
        value: *mut crate::System::Xml::Schema::SyntaxTreeNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RightChild", (value))?;
        Ok(__cordl_ret)
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+InteriorNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::InteriorNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
