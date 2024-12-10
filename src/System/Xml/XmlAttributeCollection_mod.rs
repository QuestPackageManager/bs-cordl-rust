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
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = __cordl_object
            .invoke("AddNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn Append(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("Append", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn Detach(
        &mut self,
        attr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Detach", (attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNodeOffsetNS(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindNodeOffsetNS", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertNodeAt(
        &mut self,
        i: i32,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = __cordl_object
            .invoke("InsertNodeAt", (i, node))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertParentIntoElementIdAttrMap(
        &mut self,
        attr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertParentIntoElementIdAttrMap", (attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalAppendAttribute(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("InternalAppendAttribute", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        parent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent))?;
        Ok(__cordl_object.into())
    }
    pub fn PrepareParentInElementIdAttrMap(
        &mut self,
        attrPrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrLocalName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PrepareParentInElementIdAttrMap", (attrPrefix, attrLocalName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("Remove", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("RemoveAt", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDuplicateAttribute(
        &mut self,
        attr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("RemoveDuplicateAttribute", (attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveNodeAt(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = __cordl_object
            .invoke("RemoveNodeAt", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveParentFromElementIdAttrMap(
        &mut self,
        attr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveParentFromElementIdAttrMap", (attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetParentInElementIdAttrMap(
        &mut self,
        oldVal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newVal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetParentInElementIdAttrMap", (oldVal, newVal))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNamedItem(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode> = __cordl_object
            .invoke("SetNamedItem", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.ICollection.CopyTo", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_Count(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.ICollection.get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_IsSynchronized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.ICollection.get_IsSynchronized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.ICollection.get_SyncRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemOf_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("get_ItemOf", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemOf_Il2CppString_Il2CppString2(
        &mut self,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namespaceURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("get_ItemOf", (localName, namespaceURI))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemOf_i32_0(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlAttribute> = __cordl_object
            .invoke("get_ItemOf", (i))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Xml+XmlAttributeCollection")]
impl AsRef<crate::System::Collections::ICollection>
for crate::System::Xml::XmlAttributeCollection {
    fn as_ref(&self) -> &crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XmlAttributeCollection")]
impl AsMut<crate::System::Collections::ICollection>
for crate::System::Xml::XmlAttributeCollection {
    fn as_mut(&mut self) -> &mut crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XmlAttributeCollection")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::System::Xml::XmlAttributeCollection {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XmlAttributeCollection")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::System::Xml::XmlAttributeCollection {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
