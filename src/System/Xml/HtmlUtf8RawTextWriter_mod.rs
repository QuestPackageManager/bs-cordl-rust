#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct HtmlUtf8RawTextWriter {
    __cordl_parent: crate::System::Xml::XmlUtf8RawTextWriter,
    pub elementScope: *mut crate::System::Xml::ByteStack,
    pub currentElementProperties: crate::System::Xml::ElementProperties,
    pub currentAttributeProperties: crate::System::Xml::AttributeProperties,
    pub endsWithAmpersand: bool,
    pub uriEscapingBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mediaType: *mut crate::System::String,
    pub doNotEscapeUriAttributes: bool,
}
#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::HtmlUtf8RawTextWriter =>
    "System.Xml"."HtmlUtf8RawTextWriter"
);
#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriter")]
impl std::ops::Deref for crate::System::Xml::HtmlUtf8RawTextWriter {
    type Target = crate::System::Xml::XmlUtf8RawTextWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriter")]
impl std::ops::DerefMut for crate::System::Xml::HtmlUtf8RawTextWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriter")]
impl crate::System::Xml::HtmlUtf8RawTextWriter {
    pub fn WriteUriAttributeText(
        &mut self,
        pSrc: *mut quest_hook::libil2cpp::Il2CppObject,
        pSrcEnd: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteUriAttributeText", (pSrc, pSrcEnd))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn WriteXmlDeclaration_String1(
        &mut self,
        xmldecl: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteXmlDeclaration", (xmldecl))?;
        Ok(__cordl_ret)
    }
    pub fn WriteDocType(
        &mut self,
        name: *mut crate::System::String,
        pubid: *mut crate::System::String,
        sysid: *mut crate::System::String,
        subset: *mut crate::System::String,
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
    pub fn StartElementContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartElementContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteHtmlAttributeTextBlock(
        &mut self,
        pSrc: *mut quest_hook::libil2cpp::Il2CppObject,
        pSrcEnd: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteHtmlAttributeTextBlock", (pSrc, pSrcEnd))?;
        Ok(__cordl_ret)
    }
    pub fn WriteHtmlAttributeText(
        &mut self,
        pSrc: *mut quest_hook::libil2cpp::Il2CppObject,
        pSrcEnd: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteHtmlAttributeText", (pSrc, pSrcEnd))?;
        Ok(__cordl_ret)
    }
    pub fn WriteProcessingInstruction(
        &mut self,
        target: *mut crate::System::String,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteProcessingInstruction", (target, text))?;
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
    pub fn WriteHtmlElementTextBlock(
        &mut self,
        pSrc: *mut quest_hook::libil2cpp::Il2CppObject,
        pSrcEnd: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteHtmlElementTextBlock", (pSrc, pSrcEnd))?;
        Ok(__cordl_ret)
    }
    pub fn WriteFullEndElement(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteFullEndElement", (prefix, localName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEntityRef(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEntityRef", (name))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        settings: *mut crate::System::Xml::XmlWriterSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn WriteMetaElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMetaElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn OutputRestAmps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OutputRestAmps", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartElement(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartElement", (prefix, localName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        settings: *mut crate::System::Xml::XmlWriterSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, settings))?;
        Ok(__cordl_ret)
    }
    pub fn WriteEndElement(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEndElement", (prefix, localName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartAttribute(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteStartAttribute", (prefix, localName, ns))?;
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
    pub fn WriteString(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteString", (text))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        stream: *mut crate::System::IO::Stream,
        settings: *mut crate::System::Xml::XmlWriterSettings,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, settings))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::HtmlUtf8RawTextWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
