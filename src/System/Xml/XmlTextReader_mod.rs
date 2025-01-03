#[cfg(feature = "System+Xml+XmlTextReader")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextReader {
    __cordl_parent: crate::System::Xml::XmlReader,
    pub _cordl_impl: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlTextReaderImpl>,
}
#[cfg(feature = "System+Xml+XmlTextReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReader => "System.Xml"
    ."XmlTextReader"
);
#[cfg(feature = "System+Xml+XmlTextReader")]
impl std::ops::Deref for crate::System::Xml::XmlTextReader {
    type Target = crate::System::Xml::XmlReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReader")]
impl std::ops::DerefMut for crate::System::Xml::XmlTextReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReader")]
impl crate::System::Xml::XmlTextReader {
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
    pub fn GetAttribute_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetAttribute", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttribute_Il2CppString_Il2CppString1(
        &mut self,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namespaceURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetAttribute", (localName, namespaceURI))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttribute_i32_2(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetAttribute", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasLineInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasLineInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupNamespace(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToAttribute_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToAttribute", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToAttribute_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveToAttribute", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToElement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToFirstAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToFirstAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToNextAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToNextAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString_Stream_XmlNameTable1(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url, input, nt))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream0(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextReader2(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextReader_XmlNameTable3(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, nt))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAttributeValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadAttributeValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ReadString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveEntity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveEntity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Skip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Skip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IXmlNamespaceResolver_GetNamespacesInScope(
        &mut self,
        scope: crate::System::Xml::XmlNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.GetNamespacesInScope", (scope))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IXmlNamespaceResolver_LookupNamespace(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.LookupNamespace", (prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IXmlNamespaceResolver_LookupPrefix(
        &mut self,
        namespaceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.LookupPrefix", (namespaceName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Stream_XmlNameTable1(
        &mut self,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url, input, nt))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream0(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextReader2(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextReader_XmlNameTable3(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
        nt: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input, nt))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AttributeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AttributeCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_BaseURI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanReadValueChunk(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanReadValueChunk", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanResolveEntity(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanResolveEntity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Depth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Depth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DtdInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdInfo> = __cordl_object
            .invoke("get_DtdInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EOF(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EOF", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Impl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlTextReaderImpl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlTextReaderImpl,
        > = __cordl_object.invoke("get_Impl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDefault(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEmptyElement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmptyElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_LocalName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable> = __cordl_object
            .invoke("get_NameTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NamespaceManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlNamespaceManager,
        > = __cordl_object.invoke("get_NamespaceManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NamespaceURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_NamespaceURI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Namespaces(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Namespaces", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Normalization(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Normalization", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Prefix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_QuoteChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("get_QuoteChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReadState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::ReadState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::ReadState = __cordl_object
            .invoke("get_ReadState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlLang(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_XmlLang", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlSpace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlSpace = __cordl_object
            .invoke("get_XmlSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EntityHandling(
        &mut self,
        value: crate::System::Xml::EntityHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EntityHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Normalization(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Normalization", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_WhitespaceHandling(
        &mut self,
        value: crate::System::Xml::WhitespaceHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WhitespaceHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_XmlResolver(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_XmlResolver", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_XmlValidatingReaderCompatibilityMode(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_XmlValidatingReaderCompatibilityMode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlTextReader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlTextReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlTextReader")]
impl AsRef<crate::System::Xml::IXmlLineInfo> for crate::System::Xml::XmlTextReader {
    fn as_ref(&self) -> &crate::System::Xml::IXmlLineInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XmlTextReader")]
impl AsMut<crate::System::Xml::IXmlLineInfo> for crate::System::Xml::XmlTextReader {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IXmlLineInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XmlTextReader")]
impl AsRef<crate::System::Xml::IXmlNamespaceResolver>
for crate::System::Xml::XmlTextReader {
    fn as_ref(&self) -> &crate::System::Xml::IXmlNamespaceResolver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XmlTextReader")]
impl AsMut<crate::System::Xml::IXmlNamespaceResolver>
for crate::System::Xml::XmlTextReader {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IXmlNamespaceResolver {
        unsafe { std::mem::transmute(self) }
    }
}
