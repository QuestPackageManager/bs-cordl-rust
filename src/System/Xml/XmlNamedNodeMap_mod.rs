#[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList+SingleObjectEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct SmallXmlNodeList_SingleObjectEnumerator {
    __cordl_parent: crate::System::Object,
    pub loneValue: *mut crate::System::Object,
    pub position: i32,
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList+SingleObjectEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::SmallXmlNodeList_SingleObjectEnumerator => "System.Xml"
    ."XmlNamedNodeMap/SmallXmlNodeList/SingleObjectEnumerator"
);
#[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList+SingleObjectEnumerator")]
impl std::ops::Deref for crate::System::Xml::SmallXmlNodeList_SingleObjectEnumerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList+SingleObjectEnumerator")]
impl std::ops::DerefMut for crate::System::Xml::SmallXmlNodeList_SingleObjectEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList+SingleObjectEnumerator")]
impl crate::System::Xml::SmallXmlNodeList_SingleObjectEnumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Current", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList+SingleObjectEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::SmallXmlNodeList_SingleObjectEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlNamedNodeMap_SmallXmlNodeList {
    pub field: *mut crate::System::Object,
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlNamedNodeMap_SmallXmlNodeList =>
    "System.Xml"."XmlNamedNodeMap/SmallXmlNodeList"
);
#[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlNamedNodeMap_SmallXmlNodeList {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList")]
impl crate::System::Xml::XmlNamedNodeMap_SmallXmlNodeList {
    #[cfg(
        feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList+SingleObjectEnumerator"
    )]
    pub type SingleObjectEnumerator = crate::System::Xml::SmallXmlNodeList_SingleObjectEnumerator;
    pub fn Add(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Insert(
        &mut self,
        index: i32,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Insert",
            (index, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveAt",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlNamedNodeMap {
    __cordl_parent: crate::System::Object,
    pub parent: *mut crate::System::Xml::XmlNode,
    pub nodes: crate::System::Xml::XmlNamedNodeMap_SmallXmlNodeList,
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlNamedNodeMap => "System.Xml"
    ."XmlNamedNodeMap"
);
#[cfg(feature = "System+Xml+XmlNamedNodeMap")]
impl std::ops::Deref for crate::System::Xml::XmlNamedNodeMap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap")]
impl std::ops::DerefMut for crate::System::Xml::XmlNamedNodeMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap")]
impl crate::System::Xml::XmlNamedNodeMap {
    #[cfg(feature = "System+Xml+XmlNamedNodeMap+SmallXmlNodeList")]
    pub type SmallXmlNodeList = crate::System::Xml::XmlNamedNodeMap_SmallXmlNodeList;
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
    pub fn AddNodeForLoad(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        doc: *mut crate::System::Xml::XmlDocument,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("AddNodeForLoad", (node, doc))?;
        Ok(__cordl_ret)
    }
    pub fn FindNodeOffset_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindNodeOffset", (name))?;
        Ok(__cordl_ret)
    }
    pub fn FindNodeOffset_String1(
        &mut self,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FindNodeOffset", (localName, namespaceURI))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNamedItem(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("GetNamedItem", (name))?;
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
    pub fn New(
        parent: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent))?;
        Ok(__cordl_object)
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
    pub fn ReplaceNodeAt(
        &mut self,
        i: i32,
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNode = __cordl_object
            .invoke("ReplaceNodeAt", (i, node))?;
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
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlNamedNodeMap")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlNamedNodeMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
