#[cfg(feature = "System+Xml+XmlEventCache")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlEventCache {
    __cordl_parent: crate::System::Xml::XmlRawWriter,
    pub pages: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Xml::XmlEventCache_XmlEvent,
        >,
    >,
    pub pageCurr: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlEventCache_XmlEvent,
    >,
    pub pageSize: i32,
    pub hasRootNode: bool,
    pub singleText: crate::System::Xml::Xsl::Runtime::StringConcat,
    pub baseUri: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Xml+XmlEventCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlEventCache => "System.Xml"
    ."XmlEventCache"
);
#[cfg(feature = "System+Xml+XmlEventCache")]
impl std::ops::Deref for crate::System::Xml::XmlEventCache {
    type Target = crate::System::Xml::XmlRawWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlEventCache")]
impl std::ops::DerefMut for crate::System::Xml::XmlEventCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlEventCache")]
impl crate::System::Xml::XmlEventCache {
    #[cfg(feature = "System+Xml+XmlEventCache+XmlEvent")]
    pub type XmlEvent = crate::System::Xml::XmlEventCache_XmlEvent;
    #[cfg(feature = "System+Xml+XmlEventCache+XmlEventType")]
    pub type XmlEventType = crate::System::Xml::XmlEventCache_XmlEventType;
    pub fn AddEvent_Il2CppObject5(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEvent", (eventType, o))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddEvent_Il2CppString1(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEvent", (eventType, s1))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddEvent_Il2CppString_Il2CppString2(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEvent", (eventType, s1, s2))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddEvent_Il2CppString_Il2CppString_Il2CppString3(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEvent", (eventType, s1, s2, s3))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddEvent_Il2CppString_Il2CppString_Il2CppString_Il2CppObject4(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEvent", (eventType, s1, s2, s3, o))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddEvent_XmlEventCache_XmlEventType0(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEvent", (eventType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EventsToWriter(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EventsToWriter", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        baseUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hasRootNode: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseUri, hasRootNode))?;
        Ok(__cordl_object.into())
    }
    pub fn NewEvent(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NewEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartElementContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartElementContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBytes(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBytes", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteBase64(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteBase64", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteBinHex(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteBinHex", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteCData(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteCData", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteCharEntity(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteCharEntity", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteChars(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteChars", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteComment(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteComment", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDocType(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pubid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sysid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteDocType", (name, pubid, sysid, subset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndBase64(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndBase64", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndElement(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndElement", (prefix, localName, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEntityRef(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEntityRef", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteFullEndElement(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteFullEndElement", (prefix, localName, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteNamespaceDeclaration(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNamespaceDeclaration", (prefix, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteProcessingInstruction(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteProcessingInstruction", (name, text))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRaw_Il2CppArray_i32_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRaw", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRaw_Il2CppString1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRaw", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartAttribute(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartAttribute", (prefix, localName, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartElement(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartElement", (prefix, localName, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteString(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteString", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteSurrogateCharEntity(
        &mut self,
        lowChar: char,
        highChar: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSurrogateCharEntity", (lowChar, highChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteWhitespace(
        &mut self,
        ws: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteWhitespace", (ws))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteXmlDeclaration_Il2CppString1(
        &mut self,
        xmldecl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteXmlDeclaration", (xmldecl))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteXmlDeclaration_XmlStandalone0(
        &mut self,
        standalone: crate::System::Xml::XmlStandalone,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteXmlDeclaration", (standalone))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        baseUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hasRootNode: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseUri, hasRootNode))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlEventCache")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlEventCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlEventCache+XmlEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlEventCache_XmlEvent {
    pub eventType: crate::System::Xml::XmlEventCache_XmlEventType,
    pub s1: *mut quest_hook::libil2cpp::Il2CppString,
    pub s2: *mut quest_hook::libil2cpp::Il2CppString,
    pub s3: *mut quest_hook::libil2cpp::Il2CppString,
    pub o: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+XmlEventCache+XmlEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlEventCache_XmlEvent =>
    "System.Xml"."XmlEventCache/XmlEvent"
);
#[cfg(feature = "System+Xml+XmlEventCache+XmlEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlEventCache_XmlEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlEventCache+XmlEvent")]
impl crate::System::Xml::XmlEventCache_XmlEvent {
    pub fn InitEvent_Il2CppObject5(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InitEvent",
            (eventType, o),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitEvent_Il2CppString1(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InitEvent",
            (eventType, s1),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitEvent_Il2CppString_Il2CppString2(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InitEvent",
            (eventType, s1, s2),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitEvent_Il2CppString_Il2CppString_Il2CppString3(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InitEvent",
            (eventType, s1, s2, s3),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitEvent_Il2CppString_Il2CppString_Il2CppString_Il2CppObject4(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InitEvent",
            (eventType, s1, s2, s3, o),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitEvent_XmlEventCache_XmlEventType0(
        &mut self,
        eventType: crate::System::Xml::XmlEventCache_XmlEventType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InitEvent",
            (eventType),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlEventCache_XmlEventType> {
        let __cordl_ret: crate::System::Xml::XmlEventCache_XmlEventType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_EventType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_String1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_String1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_String2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_String2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_String3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_String3", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlEventCache+XmlEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlEventCache_XmlEventType {
    Base64 = 14i32,
    BinHex = 15i32,
    CData = 5i32,
    CharEnt = 12i32,
    Close = 23i32,
    Comment = 6i32,
    Dispose = 25i32,
    DocType = 1i32,
    EndAttr = 4i32,
    EndBase64 = 22i32,
    EndElem = 19i32,
    EntRef = 11i32,
    Flush = 24i32,
    FullEndElem = 20i32,
    Nmsp = 21i32,
    PI = 7i32,
    Raw = 10i32,
    StartAttr = 3i32,
    StartContent = 18i32,
    StartElem = 2i32,
    String = 9i32,
    SurrCharEnt = 13i32,
    Unknown = 0i32,
    Whitespace = 8i32,
    XmlDecl1 = 16i32,
    XmlDecl2 = 17i32,
}
#[cfg(feature = "System+Xml+XmlEventCache+XmlEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlEventCache_XmlEventType =>
    "System.Xml"."XmlEventCache/XmlEventType"
);
