#[cfg(feature = "System+Xml+XmlTextWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextWriter {
    __cordl_parent: crate::System::Xml::XmlWriter,
    pub textWriter: *mut crate::System::IO::TextWriter,
    pub xmlEncoder: *mut crate::System::Xml::XmlTextEncoder,
    pub encoding: *mut crate::System::Text::Encoding,
    pub formatting: crate::System::Xml::Formatting,
    pub indented: bool,
    pub indentation: i32,
    pub indentChar: char,
    pub stack: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlTextWriter_TagInfo,
    >,
    pub top: i32,
    pub stateTable: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlTextWriter_State,
    >,
    pub currentState: crate::System::Xml::XmlTextWriter_State,
    pub lastToken: crate::System::Xml::XmlTextWriter_Token,
    pub base64Encoder: *mut crate::System::Xml::XmlTextWriterBase64Encoder,
    pub quoteChar: char,
    pub curQuoteChar: char,
    pub namespaces: bool,
    pub specialAttr: crate::System::Xml::XmlTextWriter_SpecialAttr,
    pub prefixForXmlNs: *mut quest_hook::libil2cpp::Il2CppString,
    pub flush: bool,
    pub nsStack: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlTextWriter_Namespace,
    >,
    pub nsTop: i32,
    pub nsHashtable: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        i32,
    >,
    pub useNsHashtable: bool,
    pub xmlCharType: crate::System::Xml::XmlCharType,
}
#[cfg(feature = "System+Xml+XmlTextWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextWriter => "System.Xml"
    ."XmlTextWriter"
);
#[cfg(feature = "System+Xml+XmlTextWriter")]
impl std::ops::Deref for crate::System::Xml::XmlTextWriter {
    type Target = crate::System::Xml::XmlWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextWriter")]
impl std::ops::DerefMut for crate::System::Xml::XmlTextWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextWriter")]
impl crate::System::Xml::XmlTextWriter {
    #[cfg(feature = "System+Xml+XmlTextWriter+Namespace")]
    pub type Namespace = crate::System::Xml::XmlTextWriter_Namespace;
    #[cfg(feature = "System+Xml+XmlTextWriter+NamespaceState")]
    pub type NamespaceState = crate::System::Xml::XmlTextWriter_NamespaceState;
    #[cfg(feature = "System+Xml+XmlTextWriter+SpecialAttr")]
    pub type SpecialAttr = crate::System::Xml::XmlTextWriter_SpecialAttr;
    #[cfg(feature = "System+Xml+XmlTextWriter+State")]
    pub type State = crate::System::Xml::XmlTextWriter_State;
    #[cfg(feature = "System+Xml+XmlTextWriter+TagInfo")]
    pub type TagInfo = crate::System::Xml::XmlTextWriter_TagInfo;
    #[cfg(feature = "System+Xml+XmlTextWriter+Token")]
    pub type Token = crate::System::Xml::XmlTextWriter_Token;
    pub fn AddNamespace(
        &mut self,
        prefix: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
        declared: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNamespace", (prefix, ns, declared))?;
        Ok(__cordl_ret)
    }
    pub fn AddToNamespaceHashtable(
        &mut self,
        namespaceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToNamespaceHashtable", (namespaceIndex))?;
        Ok(__cordl_ret)
    }
    pub fn AutoComplete(
        &mut self,
        token: crate::System::Xml::XmlTextWriter_Token,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoComplete", (token))?;
        Ok(__cordl_ret)
    }
    pub fn AutoCompleteAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoCompleteAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindPrefix(
        &mut self,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("FindPrefix", (ns))?;
        Ok(__cordl_ret)
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret)
    }
    pub fn FlushEncoders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushEncoders", ())?;
        Ok(__cordl_ret)
    }
    pub fn GeneratePrefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GeneratePrefix", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSpecialAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSpecialAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn Indent(
        &mut self,
        beforeEndElement: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Indent", (beforeEndElement))?;
        Ok(__cordl_ret)
    }
    pub fn InternalWriteEndElement(
        &mut self,
        longFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWriteEndElement", (longFormat))?;
        Ok(__cordl_ret)
    }
    pub fn InternalWriteProcessingInstruction(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        text: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWriteProcessingInstruction", (name, text))?;
        Ok(__cordl_ret)
    }
    pub fn LookupNamespace(
        &mut self,
        prefix: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn LookupNamespaceInCurrentScope(
        &mut self,
        prefix: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LookupNamespaceInCurrentScope", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn LookupPrefix(
        &mut self,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("LookupPrefix", (ns))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_Encoding2(
        filename: *mut quest_hook::libil2cpp::Il2CppString,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (filename, encoding))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_Encoding1(
        w: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (w, encoding))?;
        Ok(__cordl_object)
    }
    pub fn New_TextWriter3(
        w: *mut crate::System::IO::TextWriter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (w))?;
        Ok(__cordl_object)
    }
    pub fn PopNamespaces(
        &mut self,
        indexFrom: i32,
        indexTo: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopNamespaces", (indexFrom, indexTo))?;
        Ok(__cordl_ret)
    }
    pub fn PushNamespace(
        &mut self,
        prefix: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
        declared: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushNamespace", (prefix, ns, declared))?;
        Ok(__cordl_ret)
    }
    pub fn PushStack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartDocument(
        &mut self,
        standalone: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartDocument", (standalone))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateName(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        isNCName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateName", (name, isNCName))?;
        Ok(__cordl_ret)
    }
    pub fn VerifyPrefixXml(
        &mut self,
        prefix: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("VerifyPrefixXml", (prefix, ns))?;
        Ok(__cordl_ret)
    }
    pub fn WriteBase64(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteBase64", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn WriteBinHex(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteBinHex", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn WriteCData(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteCData", (text))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn WriteChars(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteChars", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn WriteComment(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteComment", (text))?;
        Ok(__cordl_ret)
    }
    pub fn WriteDocType(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        pubid: *mut quest_hook::libil2cpp::Il2CppString,
        sysid: *mut quest_hook::libil2cpp::Il2CppString,
        subset: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteDocType", (name, pubid, sysid, subset))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEndAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteEndAttributeQuote(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndAttributeQuote", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteEndDocument(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndDocument", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteEndStartTag(
        &mut self,
        empty: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndStartTag", (empty))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEntityRef(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEntityRef", (name))?;
        Ok(__cordl_ret)
    }
    pub fn WriteFullEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteFullEndElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteProcessingInstruction(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        text: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteProcessingInstruction", (name, text))?;
        Ok(__cordl_ret)
    }
    pub fn WriteRaw_Il2CppArray_i32_i32_0(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRaw", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn WriteRaw_Il2CppString1(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteRaw", (data))?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartAttribute(
        &mut self,
        prefix: *mut quest_hook::libil2cpp::Il2CppString,
        localName: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartAttribute", (prefix, localName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartDocument_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartDocument", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartDocument__cordl_bool1(
        &mut self,
        standalone: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartDocument", (standalone))?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartElement(
        &mut self,
        prefix: *mut quest_hook::libil2cpp::Il2CppString,
        localName: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartElement", (prefix, localName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn WriteString(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteString", (text))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn WriteWhitespace(
        &mut self,
        ws: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteWhitespace", (ws))?;
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
    pub fn _ctor_Il2CppString_Encoding2(
        &mut self,
        filename: *mut quest_hook::libil2cpp::Il2CppString,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (filename, encoding))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_Encoding1(
        &mut self,
        w: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (w, encoding))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TextWriter3(
        &mut self,
        w: *mut crate::System::IO::TextWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (w))?;
        Ok(__cordl_ret)
    }
    pub fn get_BaseStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("get_BaseStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WriteState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::WriteState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::WriteState = __cordl_object
            .invoke("get_WriteState", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Formatting(
        &mut self,
        value: crate::System::Xml::Formatting,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Formatting", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Namespaces(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Namespaces", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_QuoteChar(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_QuoteChar", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlTextWriter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlTextWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlTextWriter+Namespace")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlTextWriter_Namespace {
    pub prefix: *mut quest_hook::libil2cpp::Il2CppString,
    pub ns: *mut quest_hook::libil2cpp::Il2CppString,
    pub declared: bool,
    pub prevNsIndex: i32,
}
#[cfg(feature = "System+Xml+XmlTextWriter+Namespace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextWriter_Namespace =>
    "System.Xml"."XmlTextWriter/Namespace"
);
#[cfg(feature = "System+Xml+XmlTextWriter+Namespace")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlTextWriter_Namespace {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlTextWriter+Namespace")]
impl crate::System::Xml::XmlTextWriter_Namespace {
    pub fn Set(
        &mut self,
        prefix: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
        declared: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (prefix, ns, declared),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlTextWriter+NamespaceState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTextWriter_NamespaceState {
    DeclaredAndWrittenOut = 3i32,
    DeclaredButNotWrittenOut = 2i32,
    NotDeclaredButInScope = 1i32,
    Uninitialized = 0i32,
}
#[cfg(feature = "System+Xml+XmlTextWriter+NamespaceState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextWriter_NamespaceState =>
    "System.Xml"."XmlTextWriter/NamespaceState"
);
#[cfg(feature = "System+Xml+XmlTextWriter+SpecialAttr")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTextWriter_SpecialAttr {
    None = 0i32,
    XmlLang = 2i32,
    XmlNs = 3i32,
    XmlSpace = 1i32,
}
#[cfg(feature = "System+Xml+XmlTextWriter+SpecialAttr")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextWriter_SpecialAttr =>
    "System.Xml"."XmlTextWriter/SpecialAttr"
);
#[cfg(feature = "System+Xml+XmlTextWriter+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTextWriter_State {
    AttrOnly = 6i32,
    Attribute = 4i32,
    Closed = 9i32,
    Content = 5i32,
    Element = 3i32,
    Epilog = 7i32,
    Error = 8i32,
    PostDTD = 2i32,
    Prolog = 1i32,
    Start = 0i32,
}
#[cfg(feature = "System+Xml+XmlTextWriter+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextWriter_State => "System.Xml"
    ."XmlTextWriter/State"
);
#[cfg(feature = "System+Xml+XmlTextWriter+TagInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlTextWriter_TagInfo {
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub prefix: *mut quest_hook::libil2cpp::Il2CppString,
    pub defaultNs: *mut quest_hook::libil2cpp::Il2CppString,
    pub defaultNsState: crate::System::Xml::XmlTextWriter_NamespaceState,
    pub xmlSpace: crate::System::Xml::XmlSpace,
    pub xmlLang: *mut quest_hook::libil2cpp::Il2CppString,
    pub prevNsTop: i32,
    pub prefixCount: i32,
    pub mixed: bool,
}
#[cfg(feature = "System+Xml+XmlTextWriter+TagInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextWriter_TagInfo =>
    "System.Xml"."XmlTextWriter/TagInfo"
);
#[cfg(feature = "System+Xml+XmlTextWriter+TagInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlTextWriter_TagInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlTextWriter+TagInfo")]
impl crate::System::Xml::XmlTextWriter_TagInfo {
    pub fn Init(
        &mut self,
        nsTop: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (nsTop),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlTextWriter+Token")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTextWriter_Token {
    Base64 = 10i32,
    CData = 3i32,
    Comment = 2i32,
    Content = 9i32,
    Doctype = 1i32,
    Empty = 13i32,
    EndAttribute = 8i32,
    EndElement = 5i32,
    LongEndElement = 6i32,
    PI = 0i32,
    RawData = 11i32,
    StartAttribute = 7i32,
    StartElement = 4i32,
    Whitespace = 12i32,
}
#[cfg(feature = "System+Xml+XmlTextWriter+Token")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextWriter_Token => "System.Xml"
    ."XmlTextWriter/Token"
);
