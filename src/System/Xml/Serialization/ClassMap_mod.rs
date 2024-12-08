#[cfg(feature = "System+Xml+Serialization+ClassMap")]
#[repr(C)]
#[derive(Debug)]
pub struct ClassMap {
    __cordl_parent: crate::System::Xml::Serialization::ObjectMap,
    pub _elements: *mut crate::System::Collections::Hashtable,
    pub _elementMembers: *mut crate::System::Collections::ArrayList,
    pub _attributeMembers: *mut crate::System::Collections::Hashtable,
    pub _attributeMembersArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::Serialization::XmlTypeMapMemberAttribute,
    >,
    pub _flatLists: *mut crate::System::Collections::ArrayList,
    pub _allMembers: *mut crate::System::Collections::ArrayList,
    pub _membersWithDefault: *mut crate::System::Collections::ArrayList,
    pub _listMembers: *mut crate::System::Collections::ArrayList,
    pub _defaultAnyElement: *mut crate::System::Xml::Serialization::XmlTypeMapMemberAnyElement,
    pub _defaultAnyAttribute: *mut crate::System::Xml::Serialization::XmlTypeMapMemberAnyAttribute,
    pub _namespaceDeclarations: *mut crate::System::Xml::Serialization::XmlTypeMapMemberNamespaces,
    pub _xmlTextCollector: *mut crate::System::Xml::Serialization::XmlTypeMapMember,
    pub _returnMember: *mut crate::System::Xml::Serialization::XmlTypeMapMember,
    pub _ignoreMemberNamespace: bool,
    pub _canBeSimpleType: bool,
    pub _isOrderDependentMap: crate::System::Nullable_1<bool>,
}
#[cfg(feature = "System+Xml+Serialization+ClassMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::ClassMap =>
    "System.Xml.Serialization"."ClassMap"
);
#[cfg(feature = "System+Xml+Serialization+ClassMap")]
impl std::ops::Deref for crate::System::Xml::Serialization::ClassMap {
    type Target = crate::System::Xml::Serialization::ObjectMap;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+ClassMap")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::ClassMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+ClassMap")]
impl crate::System::Xml::Serialization::ClassMap {
    pub fn AddMember(
        &mut self,
        member: *mut crate::System::Xml::Serialization::XmlTypeMapMember,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddMember", (member))?;
        Ok(__cordl_ret)
    }
    pub fn BuildKey(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        explicitOrder: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("BuildKey", (name, ns, explicitOrder))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttribute(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapMemberAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapMemberAttribute = __cordl_object
            .invoke("GetAttribute", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn GetElement_String_String1(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo = __cordl_object
            .invoke("GetElement", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn GetElement_i32_0(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        minimalOrder: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo = __cordl_object
            .invoke("GetElement", (name, ns, minimalOrder))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RegisterFlatList(
        &mut self,
        member: *mut crate::System::Xml::Serialization::XmlTypeMapMemberExpandable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterFlatList", (member))?;
        Ok(__cordl_ret)
    }
    pub fn SetCanBeSimpleType(
        &mut self,
        can: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCanBeSimpleType", (can))?;
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
    pub fn get_AllMembers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_AllMembers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AttributeMembers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("get_AttributeMembers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultAnyAttributeMember(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapMemberAnyAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapMemberAnyAttribute = __cordl_object
            .invoke("get_DefaultAnyAttributeMember", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultAnyElementMember(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapMemberAnyElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapMemberAnyElement = __cordl_object
            .invoke("get_DefaultAnyElementMember", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ElementMembers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("get_ElementMembers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FlatLists(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_FlatLists", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasSimpleContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasSimpleContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsOrderDependentMap(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsOrderDependentMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ListMembers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_ListMembers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NamespaceDeclarations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapMemberNamespaces,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapMemberNamespaces = __cordl_object
            .invoke("get_NamespaceDeclarations", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReturnMember(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapMember,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapMember = __cordl_object
            .invoke("get_ReturnMember", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SimpleContentBaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("get_SimpleContentBaseType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlTextCollector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapMember,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapMember = __cordl_object
            .invoke("get_XmlTextCollector", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+ClassMap")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Serialization::ClassMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}