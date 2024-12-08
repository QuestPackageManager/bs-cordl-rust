#[cfg(feature = "System+Xml+IDtdParserAdapter")]
#[repr(C)]
#[derive(Debug)]
pub struct IDtdParserAdapter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+IDtdParserAdapter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::IDtdParserAdapter => "System.Xml"
    ."IDtdParserAdapter"
);
#[cfg(feature = "System+Xml+IDtdParserAdapter")]
impl std::ops::Deref for crate::System::Xml::IDtdParserAdapter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapter")]
impl std::ops::DerefMut for crate::System::Xml::IDtdParserAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapter")]
impl crate::System::Xml::IDtdParserAdapter {
    pub fn get_LineStartPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineStartPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEof(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEof", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEntityEolNormalized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEntityEolNormalized", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseNumericCharRef(
        &mut self,
        internalSubsetBuilder: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ParseNumericCharRef", (internalSubsetBuilder))?;
        Ok(__cordl_ret)
    }
    pub fn ParsePI(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParsePI", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn ParseComment(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseComment", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn get_LineNo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNo", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopEntity(
        &mut self,
        oldEntity: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::IDtdEntityInfo,
        >,
        newEntityId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PopEntity", (oldEntity, newEntityId))?;
        Ok(__cordl_ret)
    }
    pub fn PushInternalDtd(
        &mut self,
        baseUri: *mut crate::System::String,
        internalDtd: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushInternalDtd", (baseUri, internalDtd))?;
        Ok(__cordl_ret)
    }
    pub fn OnSystemId(
        &mut self,
        systemId: *mut crate::System::String,
        keywordLineInfo: crate::System::Xml::LineInfo,
        systemLiteralLineInfo: crate::System::Xml::LineInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSystemId", (systemId, keywordLineInfo, systemLiteralLineInfo))?;
        Ok(__cordl_ret)
    }
    pub fn Throw(
        &mut self,
        e: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnPublicId(
        &mut self,
        publicId: *mut crate::System::String,
        keywordLineInfo: crate::System::Xml::LineInfo,
        publicLiteralLineInfo: crate::System::Xml::LineInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPublicId", (publicId, keywordLineInfo, publicLiteralLineInfo))?;
        Ok(__cordl_ret)
    }
    pub fn OnNewLine(
        &mut self,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNewLine", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn get_BaseUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("get_BaseUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CurrentPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CurrentPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_CurrentPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CurrentPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParsingBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<char> = __cordl_object
            .invoke("get_ParsingBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseNamedCharRef(
        &mut self,
        expand: bool,
        internalSubsetBuilder: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ParseNamedCharRef", (expand, internalSubsetBuilder))?;
        Ok(__cordl_ret)
    }
    pub fn PushEntity(
        &mut self,
        entity: *mut crate::System::Xml::IDtdEntityInfo,
        entityId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PushEntity", (entity, entityId))?;
        Ok(__cordl_ret)
    }
    pub fn get_NamespaceResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::IXmlNamespaceResolver> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IXmlNamespaceResolver = __cordl_object
            .invoke("get_NamespaceResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn PushExternalSubset(
        &mut self,
        systemId: *mut crate::System::String,
        publicId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PushExternalSubset", (systemId, publicId))?;
        Ok(__cordl_ret)
    }
    pub fn get_ParsingBufferLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ParsingBufferLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadData(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNameTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNameTable = __cordl_object
            .invoke("get_NameTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EntityStackLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_EntityStackLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::IDtdParserAdapter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
