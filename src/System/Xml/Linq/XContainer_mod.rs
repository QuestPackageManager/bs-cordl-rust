#[cfg(feature = "System+Xml+Linq+XContainer")]
#[repr(C)]
#[derive(Debug)]
pub struct XContainer {
    __cordl_parent: crate::System::Xml::Linq::XNode,
    pub content: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+Linq+XContainer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::XContainer =>
    "System.Xml.Linq"."XContainer"
);
#[cfg(feature = "System+Xml+Linq+XContainer")]
impl std::ops::Deref for crate::System::Xml::Linq::XContainer {
    type Target = crate::System::Xml::Linq::XNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XContainer")]
impl std::ops::DerefMut for crate::System::Xml::Linq::XContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XContainer")]
impl crate::System::Xml::Linq::XContainer {
    #[cfg(feature = "System+Xml+Linq+XContainer+ContentReader")]
    pub type ContentReader = crate::System::Xml::Linq::XContainer_ContentReader;
    #[cfg(feature = "System+Xml+Linq+XContainer+_Nodes_d__18")]
    pub type _Nodes_d__18 = crate::System::Xml::Linq::XContainer__Nodes_d__18;
    pub fn Add(
        &mut self,
        content: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (content))?;
        Ok(__cordl_ret)
    }
    pub fn AddAttribute(
        &mut self,
        a: *mut crate::System::Xml::Linq::XAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttribute", (a))?;
        Ok(__cordl_ret)
    }
    pub fn AddAttributeSkipNotify(
        &mut self,
        a: *mut crate::System::Xml::Linq::XAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttributeSkipNotify", (a))?;
        Ok(__cordl_ret)
    }
    pub fn AddContentSkipNotify(
        &mut self,
        content: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddContentSkipNotify", (content))?;
        Ok(__cordl_ret)
    }
    pub fn AddNode(
        &mut self,
        n: *mut crate::System::Xml::Linq::XNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNode", (n))?;
        Ok(__cordl_ret)
    }
    pub fn AddNodeSkipNotify(
        &mut self,
        n: *mut crate::System::Xml::Linq::XNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNodeSkipNotify", (n))?;
        Ok(__cordl_ret)
    }
    pub fn AddString(
        &mut self,
        s: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddString", (s))?;
        Ok(__cordl_ret)
    }
    pub fn AddStringSkipNotify(
        &mut self,
        s: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStringSkipNotify", (s))?;
        Ok(__cordl_ret)
    }
    pub fn AppendNode(
        &mut self,
        n: *mut crate::System::Xml::Linq::XNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendNode", (n))?;
        Ok(__cordl_ret)
    }
    pub fn AppendNodeSkipNotify(
        &mut self,
        n: *mut crate::System::Xml::Linq::XNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendNodeSkipNotify", (n))?;
        Ok(__cordl_ret)
    }
    pub fn AppendText(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendText", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertTextToNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertTextToNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_XContainer1(
        other: *mut crate::System::Xml::Linq::XContainer,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other))?;
        Ok(__cordl_object)
    }
    pub fn Nodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Xml::Linq::XNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Xml::Linq::XNode,
        > = __cordl_object.invoke("Nodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadContentFrom_LoadOptions1(
        &mut self,
        r: *mut crate::System::Xml::XmlReader,
        o: crate::System::Xml::Linq::LoadOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadContentFrom", (r, o))?;
        Ok(__cordl_ret)
    }
    pub fn ReadContentFrom_XmlReader0(
        &mut self,
        r: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadContentFrom", (r))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveNode(
        &mut self,
        n: *mut crate::System::Xml::Linq::XNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveNode", (n))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveNodesSkipNotify(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveNodesSkipNotify", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateNode(
        &mut self,
        node: *mut crate::System::Xml::Linq::XNode,
        previous: *mut crate::System::Xml::Linq::XNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateNode", (node, previous))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateString(
        &mut self,
        s: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateString", (s))?;
        Ok(__cordl_ret)
    }
    pub fn WriteContentTo(
        &mut self,
        writer: *mut crate::System::Xml::XmlWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteContentTo", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XContainer1(
        &mut self,
        other: *mut crate::System::Xml::Linq::XContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other))?;
        Ok(__cordl_ret)
    }
    pub fn get_LastNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Linq::XNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Linq::XNode = __cordl_object
            .invoke("get_LastNode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Linq+XContainer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Linq::XContainer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Linq+XContainer+ContentReader")]
#[repr(C)]
#[derive(Debug)]
pub struct XContainer_ContentReader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _eCache: crate::System::Xml::Linq::NamespaceCache,
    pub _aCache: crate::System::Xml::Linq::NamespaceCache,
    pub _lineInfo: *mut crate::System::Xml::IXmlLineInfo,
    pub _currentContainer: *mut crate::System::Xml::Linq::XContainer,
    pub _baseUri: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Xml+Linq+XContainer+ContentReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::XContainer_ContentReader =>
    "System.Xml.Linq"."XContainer/ContentReader"
);
#[cfg(feature = "System+Xml+Linq+XContainer+ContentReader")]
impl std::ops::Deref for crate::System::Xml::Linq::XContainer_ContentReader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XContainer+ContentReader")]
impl std::ops::DerefMut for crate::System::Xml::Linq::XContainer_ContentReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XContainer+ContentReader")]
impl crate::System::Xml::Linq::XContainer_ContentReader {
    pub fn New_XContainer0(
        rootContainer: *mut crate::System::Xml::Linq::XContainer,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rootContainer))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlReader_LoadOptions1(
        rootContainer: *mut crate::System::Xml::Linq::XContainer,
        r: *mut crate::System::Xml::XmlReader,
        o: crate::System::Xml::Linq::LoadOptions,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rootContainer, r, o))?;
        Ok(__cordl_object)
    }
    pub fn ReadContentFrom_LoadOptions1(
        &mut self,
        rootContainer: *mut crate::System::Xml::Linq::XContainer,
        r: *mut crate::System::Xml::XmlReader,
        o: crate::System::Xml::Linq::LoadOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReadContentFrom", (rootContainer, r, o))?;
        Ok(__cordl_ret)
    }
    pub fn ReadContentFrom_XContainer_XmlReader0(
        &mut self,
        rootContainer: *mut crate::System::Xml::Linq::XContainer,
        r: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReadContentFrom", (rootContainer, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XContainer0(
        &mut self,
        rootContainer: *mut crate::System::Xml::Linq::XContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rootContainer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlReader_LoadOptions1(
        &mut self,
        rootContainer: *mut crate::System::Xml::Linq::XContainer,
        r: *mut crate::System::Xml::XmlReader,
        o: crate::System::Xml::Linq::LoadOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rootContainer, r, o))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Linq+XContainer+ContentReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Linq::XContainer_ContentReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
