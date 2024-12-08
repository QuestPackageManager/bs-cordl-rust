#[cfg(feature = "Mono+Xml+SecurityParser")]
#[repr(C)]
#[derive(Debug)]
pub struct SecurityParser {
    __cordl_parent: crate::Mono::Xml::SmallXmlParser,
    pub root: *mut crate::System::Security::SecurityElement,
    pub current: *mut crate::System::Security::SecurityElement,
    pub stack: *mut crate::System::Collections::Stack,
}
#[cfg(feature = "Mono+Xml+SecurityParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Xml::SecurityParser => "Mono.Xml"
    ."SecurityParser"
);
#[cfg(feature = "Mono+Xml+SecurityParser")]
impl std::ops::Deref for crate::Mono::Xml::SecurityParser {
    type Target = crate::Mono::Xml::SmallXmlParser;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Xml+SecurityParser")]
impl std::ops::DerefMut for crate::Mono::Xml::SecurityParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Xml+SecurityParser")]
impl crate::Mono::Xml::SecurityParser {
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
    pub fn OnEndElement(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEndElement", (name))?;
        Ok(__cordl_ret)
    }
    pub fn OnEndParsing(
        &mut self,
        parser: *mut crate::Mono::Xml::SmallXmlParser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEndParsing", (parser))?;
        Ok(__cordl_ret)
    }
    pub fn OnStartElement(
        &mut self,
        name: *mut crate::System::String,
        attrs: *mut crate::Mono::Xml::SmallXmlParser_IAttrList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStartElement", (name, attrs))?;
        Ok(__cordl_ret)
    }
    pub fn LoadXml(
        &mut self,
        xml: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadXml", (xml))?;
        Ok(__cordl_ret)
    }
    pub fn OnIgnorableWhitespace(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnIgnorableWhitespace", (s))?;
        Ok(__cordl_ret)
    }
    pub fn ToXml(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Security::SecurityElement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::SecurityElement = __cordl_object
            .invoke("ToXml", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnChars(
        &mut self,
        ch: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnChars", (ch))?;
        Ok(__cordl_ret)
    }
    pub fn OnStartParsing(
        &mut self,
        parser: *mut crate::Mono::Xml::SmallXmlParser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStartParsing", (parser))?;
        Ok(__cordl_ret)
    }
    pub fn OnProcessingInstruction(
        &mut self,
        name: *mut crate::System::String,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnProcessingInstruction", (name, text))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Xml+SecurityParser")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Xml::SecurityParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
