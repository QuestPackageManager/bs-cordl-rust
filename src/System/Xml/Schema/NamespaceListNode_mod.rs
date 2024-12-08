#[cfg(feature = "System+Xml+Schema+NamespaceListNode")]
#[repr(C)]
#[derive(Debug)]
pub struct NamespaceListNode {
    __cordl_parent: crate::System::Xml::Schema::SyntaxTreeNode,
    pub namespaceList: *mut crate::System::Xml::Schema::NamespaceList,
    pub particle: *mut crate::System::Object,
}
#[cfg(feature = "System+Xml+Schema+NamespaceListNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::NamespaceListNode =>
    "System.Xml.Schema"."NamespaceListNode"
);
#[cfg(feature = "System+Xml+Schema+NamespaceListNode")]
impl std::ops::Deref for crate::System::Xml::Schema::NamespaceListNode {
    type Target = crate::System::Xml::Schema::SyntaxTreeNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+NamespaceListNode")]
impl std::ops::DerefMut for crate::System::Xml::Schema::NamespaceListNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+NamespaceListNode")]
impl crate::System::Xml::Schema::NamespaceListNode {
    pub fn _ctor(
        &mut self,
        namespaceList: *mut crate::System::Xml::Schema::NamespaceList,
        particle: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namespaceList, particle))?;
        Ok(__cordl_ret)
    }
    pub fn ConstructPos(
        &mut self,
        firstpos: *mut crate::System::Xml::Schema::BitSet,
        lastpos: *mut crate::System::Xml::Schema::BitSet,
        followpos: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::BitSet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConstructPos", (firstpos, lastpos, followpos))?;
        Ok(__cordl_ret)
    }
    pub fn GetResolvedSymbols(
        &mut self,
        symbols: *mut crate::System::Xml::Schema::SymbolsDictionary,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("GetResolvedSymbols", (symbols))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNullable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNullable", ())?;
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
    pub fn New(
        namespaceList: *mut crate::System::Xml::Schema::NamespaceList,
        particle: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namespaceList, particle))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+NamespaceListNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::NamespaceListNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
