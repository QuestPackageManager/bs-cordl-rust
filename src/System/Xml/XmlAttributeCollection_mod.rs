#[cfg(feature = "System+Xml+XmlAttributeCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAttributeCollection {
    __cordl_parent: crate::System::Xml::XmlNamedNodeMap,
}
#[cfg(feature = "System+Xml+XmlAttributeCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlAttributeCollection =>
    "System.Xml"."XmlAttributeCollection"
);
#[cfg(feature = "System+Xml+XmlAttributeCollection")]
impl std::ops::Deref for crate::System::Xml::XmlAttributeCollection {
    type Target = crate::System::Xml::XmlNamedNodeMap;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlAttributeCollection")]
impl std::ops::DerefMut for crate::System::Xml::XmlAttributeCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlAttributeCollection")]
impl crate::System::Xml::XmlAttributeCollection {
    pub fn AddNode(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("AddNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn Append(
        &mut self,
        node: *mut crate::System::Xml::XmlAttribute,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("Append", (node))?;
        Ok(__cordl_ret)
    }
    pub fn Detach(
        &mut self,
        attr: *mut crate::System::Xml::XmlAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Detach", (attr))?;
        Ok(__cordl_ret)
    }
    pub fn FindNodeOffsetNS(
        &mut self,
        node: *mut crate::System::Xml::XmlAttribute,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindNodeOffsetNS", (node))?;
        Ok(__cordl_ret)
    }
    pub fn InsertNodeAt(
        &mut self,
        i: i32,
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("InsertNodeAt", (i, node))?;
        Ok(__cordl_ret)
    }
    pub fn InsertParentIntoElementIdAttrMap(
        &mut self,
        attr: *mut crate::System::Xml::XmlAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertParentIntoElementIdAttrMap", (attr))?;
        Ok(__cordl_ret)
    }
    pub fn InternalAppendAttribute(
        &mut self,
        node: *mut crate::System::Xml::XmlAttribute,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("InternalAppendAttribute", (node))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parent: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent))?;
        Ok(__cordl_object)
    }
    pub fn PrepareParentInElementIdAttrMap(
        &mut self,
        attrPrefix: *mut crate::System::String,
        attrLocalName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PrepareParentInElementIdAttrMap", (attrPrefix, attrLocalName))?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        node: *mut crate::System::Xml::XmlAttribute,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("Remove", (node))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveAt(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("RemoveAt", (i))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveDuplicateAttribute(
        &mut self,
        attr: *mut crate::System::Xml::XmlAttribute,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("RemoveDuplicateAttribute", (attr))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveNodeAt(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("RemoveNodeAt", (i))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveParentFromElementIdAttrMap(
        &mut self,
        attr: *mut crate::System::Xml::XmlAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveParentFromElementIdAttrMap", (attr))?;
        Ok(__cordl_ret)
    }
    pub fn ResetParentInElementIdAttrMap(
        &mut self,
        oldVal: *mut crate::System::String,
        newVal: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetParentInElementIdAttrMap", (oldVal, newVal))?;
        Ok(__cordl_ret)
    }
    pub fn SetNamedItem(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("SetNamedItem", (node))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_ICollection_CopyTo(
        &mut self,
        array: *mut crate::System::Array,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.ICollection.CopyTo", (array, index))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_ICollection_get_Count(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.ICollection.get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_ICollection_get_IsSynchronized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.ICollection.get_IsSynchronized", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_ICollection_get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.Collections.ICollection.get_SyncRoot", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        parent: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parent))?;
        Ok(__cordl_ret)
    }
    pub fn get_ItemOf_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("get_ItemOf", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_ItemOf_String_String2(
        &mut self,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("get_ItemOf", (localName, namespaceURI))?;
        Ok(__cordl_ret)
    }
    pub fn get_ItemOf_i32_0(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlAttribute> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlAttribute = __cordl_object
            .invoke("get_ItemOf", (i))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlAttributeCollection")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlAttributeCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
