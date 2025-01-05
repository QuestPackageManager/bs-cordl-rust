#[cfg(feature = "System+Xml+Schema+InteriorNode")]
#[repr(C)]
#[derive(Debug)]
pub struct InteriorNode {
    __cordl_parent: crate::System::Xml::Schema::SyntaxTreeNode,
    pub leftChild: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SyntaxTreeNode>,
    pub rightChild: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::SyntaxTreeNode,
    >,
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
    pub fn ExpandTree(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::InteriorNode>,
        symbols: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SymbolsDictionary,
        >,
        positions: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Positions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandTree", (parent, symbols, positions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandTreeNoRecursive(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::InteriorNode>,
        symbols: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SymbolsDictionary,
        >,
        positions: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Positions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandTreeNoRecursive", (parent, symbols, positions))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LeftChild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SyntaxTreeNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SyntaxTreeNode,
        > = __cordl_object.invoke("get_LeftChild", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RightChild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SyntaxTreeNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SyntaxTreeNode,
        > = __cordl_object.invoke("get_RightChild", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LeftChild(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SyntaxTreeNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LeftChild", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RightChild(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SyntaxTreeNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RightChild", (value))?;
        Ok(__cordl_ret.into())
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
