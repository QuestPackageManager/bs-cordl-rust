#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriterIndent")]
#[repr(C)]
#[derive(Debug)]
pub struct HtmlUtf8RawTextWriterIndent {
    __cordl_parent: crate::System::Xml::HtmlUtf8RawTextWriter,
    pub indentLevel: i32,
    pub endBlockPos: i32,
    pub indentChars: *mut crate::System::String,
    pub newLineOnAttributes: bool,
}
#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriterIndent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::HtmlUtf8RawTextWriterIndent =>
    "System.Xml"."HtmlUtf8RawTextWriterIndent"
);
#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriterIndent")]
impl std::ops::Deref for crate::System::Xml::HtmlUtf8RawTextWriterIndent {
    type Target = crate::System::Xml::HtmlUtf8RawTextWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriterIndent")]
impl std::ops::DerefMut for crate::System::Xml::HtmlUtf8RawTextWriterIndent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriterIndent")]
impl crate::System::Xml::HtmlUtf8RawTextWriterIndent {
    pub fn FlushBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushBuffer", ())?;
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
    pub fn New(
        stream: *mut crate::System::IO::Stream,
        settings: *mut crate::System::Xml::XmlWriterSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, settings))?;
        Ok(__cordl_object)
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
    pub fn WriteIndent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteIndent", ())?;
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
}
#[cfg(feature = "System+Xml+HtmlUtf8RawTextWriterIndent")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::HtmlUtf8RawTextWriterIndent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}