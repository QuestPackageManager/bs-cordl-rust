#[cfg(
    feature = "System+Xml+XmlTextReaderImpl+DtdDefaultAttributeInfoToNodeDataComparer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextReaderImpl_DtdDefaultAttributeInfoToNodeDataComparer {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "System+Xml+XmlTextReaderImpl+DtdDefaultAttributeInfoToNodeDataComparer"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::XmlTextReaderImpl_DtdDefaultAttributeInfoToNodeDataComparer =>
    "System.Xml"."XmlTextReaderImpl/DtdDefaultAttributeInfoToNodeDataComparer"
);
#[cfg(
    feature = "System+Xml+XmlTextReaderImpl+DtdDefaultAttributeInfoToNodeDataComparer"
)]
impl std::ops::Deref
for crate::System::Xml::XmlTextReaderImpl_DtdDefaultAttributeInfoToNodeDataComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Xml+XmlTextReaderImpl+DtdDefaultAttributeInfoToNodeDataComparer"
)]
impl std::ops::DerefMut
for crate::System::Xml::XmlTextReaderImpl_DtdDefaultAttributeInfoToNodeDataComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Xml+XmlTextReaderImpl+DtdDefaultAttributeInfoToNodeDataComparer"
)]
impl crate::System::Xml::XmlTextReaderImpl_DtdDefaultAttributeInfoToNodeDataComparer {
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
    pub fn Compare(
        &mut self,
        x: *mut crate::System::Object,
        y: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (x, y))?;
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
#[cfg(
    feature = "System+Xml+XmlTextReaderImpl+DtdDefaultAttributeInfoToNodeDataComparer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlTextReaderImpl_DtdDefaultAttributeInfoToNodeDataComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+DtdParserProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextReaderImpl_DtdParserProxy {
    __cordl_parent: crate::System::Object,
    pub reader: *mut crate::System::Xml::XmlTextReaderImpl,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+DtdParserProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl_DtdParserProxy =>
    "System.Xml"."XmlTextReaderImpl/DtdParserProxy"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+DtdParserProxy")]
impl std::ops::Deref for crate::System::Xml::XmlTextReaderImpl_DtdParserProxy {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+DtdParserProxy")]
impl std::ops::DerefMut for crate::System::Xml::XmlTextReaderImpl_DtdParserProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+DtdParserProxy")]
impl crate::System::Xml::XmlTextReaderImpl_DtdParserProxy {
    pub fn System_Xml_IDtdParserAdapterV1_get_V1CompatibilityMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdParserAdapterV1.get_V1CompatibilityMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_ParsingBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<char> = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_ParsingBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapterWithValidation_get_DtdValidation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdParserAdapterWithValidation.get_DtdValidation", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_IsEof(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_IsEof", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_PushInternalDtd(
        &mut self,
        baseUri: *mut crate::System::String,
        internalDtd: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Xml.IDtdParserAdapter.PushInternalDtd",
                (baseUri, internalDtd),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapterWithValidation_get_ValidationEventHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::IValidationEventHandling,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IValidationEventHandling = __cordl_object
            .invoke(
                "System.Xml.IDtdParserAdapterWithValidation.get_ValidationEventHandling",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_ReadData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.ReadData", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_OnNewLine(
        &mut self,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.OnNewLine", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_NamespaceResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::IXmlNamespaceResolver> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IXmlNamespaceResolver = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_NamespaceResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_ParseComment(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.ParseComment", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNameTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNameTable = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_NameTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_LineStartPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_LineStartPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_OnSystemId(
        &mut self,
        systemId: *mut crate::System::String,
        keywordLineInfo: crate::System::Xml::LineInfo,
        systemLiteralLineInfo: crate::System::Xml::LineInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Xml.IDtdParserAdapter.OnSystemId",
                (systemId, keywordLineInfo, systemLiteralLineInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_IsEntityEolNormalized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_IsEntityEolNormalized", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_ParseNamedCharRef(
        &mut self,
        expand: bool,
        internalSubsetBuilder: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "System.Xml.IDtdParserAdapter.ParseNamedCharRef",
                (expand, internalSubsetBuilder),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_Throw(
        &mut self,
        e: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.Throw", (e))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_ParseNumericCharRef(
        &mut self,
        internalSubsetBuilder: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "System.Xml.IDtdParserAdapter.ParseNumericCharRef",
                (internalSubsetBuilder),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_LineNo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_LineNo", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_EntityStackLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_EntityStackLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_PopEntity(
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
            .invoke("System.Xml.IDtdParserAdapter.PopEntity", (oldEntity, newEntityId))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_PushEntity(
        &mut self,
        entity: *mut crate::System::Xml::IDtdEntityInfo,
        entityId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.PushEntity", (entity, entityId))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_CurrentPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_CurrentPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        reader: *mut crate::System::Xml::XmlTextReaderImpl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_set_CurrentPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.set_CurrentPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_ParsingBufferLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_ParsingBufferLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_ParsePI(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.ParsePI", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_PushExternalSubset(
        &mut self,
        systemId: *mut crate::System::String,
        publicId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Xml.IDtdParserAdapter.PushExternalSubset",
                (systemId, publicId),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapterV1_get_Normalization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdParserAdapterV1.get_Normalization", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapterV1_get_Namespaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdParserAdapterV1.get_Namespaces", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_OnPublicId(
        &mut self,
        publicId: *mut crate::System::String,
        keywordLineInfo: crate::System::Xml::LineInfo,
        publicLiteralLineInfo: crate::System::Xml::LineInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Xml.IDtdParserAdapter.OnPublicId",
                (publicId, keywordLineInfo, publicLiteralLineInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IDtdParserAdapter_get_BaseUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("System.Xml.IDtdParserAdapter.get_BaseUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        reader: *mut crate::System::Xml::XmlTextReaderImpl,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+DtdParserProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlTextReaderImpl_DtdParserProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+EntityExpandType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTextReaderImpl_EntityExpandType {
    All = 0i32,
    OnlyCharacter = 2i32,
    OnlyGeneral = 1i32,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+EntityExpandType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl_EntityExpandType
    => "System.Xml"."XmlTextReaderImpl/EntityExpandType"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+EntityType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTextReaderImpl_EntityType {
    CharacterDec = 0i32,
    CharacterHex = 1i32,
    CharacterNamed = 2i32,
    Expanded = 3i32,
    ExpandedInAttribute = 7i32,
    FakeExpanded = 5i32,
    Skipped = 4i32,
    Unexpanded = 6i32,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+EntityType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl_EntityType =>
    "System.Xml"."XmlTextReaderImpl/EntityType"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+IncrementalReadState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTextReaderImpl_IncrementalReadState {
    AttributeValue = 6i32,
    Attributes = 5i32,
    CDATA = 3i32,
    Comment = 4i32,
    End = 9i32,
    EndElement = 8i32,
    PI = 2i32,
    ReadContentAsBinary_End = 14i32,
    ReadContentAsBinary_OnCachedValue = 12i32,
    ReadContentAsBinary_OnPartialValue = 13i32,
    ReadData = 7i32,
    ReadValueChunk_OnCachedValue = 10i32,
    ReadValueChunk_OnPartialValue = 11i32,
    StartTag = 1i32,
    Text = 0i32,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+IncrementalReadState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::XmlTextReaderImpl_IncrementalReadState => "System.Xml"
    ."XmlTextReaderImpl/IncrementalReadState"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+InitInputType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTextReaderImpl_InitInputType {
    Invalid = 3i32,
    Stream = 1i32,
    TextReader = 2i32,
    UriString = 0i32,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+InitInputType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl_InitInputType =>
    "System.Xml"."XmlTextReaderImpl/InitInputType"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+LaterInitParam")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextReaderImpl_LaterInitParam {
    __cordl_parent: crate::System::Object,
    pub useAsync: bool,
    pub inputStream: *mut crate::System::IO::Stream,
    pub inputBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub inputByteCount: i32,
    pub inputbaseUri: *mut crate::System::Uri,
    pub inputUriStr: *mut crate::System::String,
    pub inputUriResolver: *mut crate::System::Xml::XmlResolver,
    pub inputContext: *mut crate::System::Xml::XmlParserContext,
    pub inputTextReader: *mut crate::System::IO::TextReader,
    pub initType: crate::System::Xml::XmlTextReaderImpl_InitInputType,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+LaterInitParam")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl_LaterInitParam =>
    "System.Xml"."XmlTextReaderImpl/LaterInitParam"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+LaterInitParam")]
impl std::ops::Deref for crate::System::Xml::XmlTextReaderImpl_LaterInitParam {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+LaterInitParam")]
impl std::ops::DerefMut for crate::System::Xml::XmlTextReaderImpl_LaterInitParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+LaterInitParam")]
impl crate::System::Xml::XmlTextReaderImpl_LaterInitParam {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+LaterInitParam")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlTextReaderImpl_LaterInitParam {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NoNamespaceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextReaderImpl_NoNamespaceManager {
    __cordl_parent: crate::System::Xml::XmlNamespaceManager,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NoNamespaceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::XmlTextReaderImpl_NoNamespaceManager => "System.Xml"
    ."XmlTextReaderImpl/NoNamespaceManager"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NoNamespaceManager")]
impl std::ops::Deref for crate::System::Xml::XmlTextReaderImpl_NoNamespaceManager {
    type Target = crate::System::Xml::XmlNamespaceManager;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NoNamespaceManager")]
impl std::ops::DerefMut for crate::System::Xml::XmlTextReaderImpl_NoNamespaceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NoNamespaceManager")]
impl crate::System::Xml::XmlTextReaderImpl_NoNamespaceManager {
    pub fn PushScope(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushScope", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn get_DefaultNamespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DefaultNamespace", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopScope(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PopScope", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNamespacesInScope(
        &mut self,
        scope: crate::System::Xml::XmlNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("GetNamespacesInScope", (scope))?;
        Ok(__cordl_ret)
    }
    pub fn LookupNamespace(
        &mut self,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn LookupPrefix(
        &mut self,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupPrefix", (uri))?;
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
    pub fn RemoveNamespace(
        &mut self,
        prefix: *mut crate::System::String,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveNamespace", (prefix, uri))?;
        Ok(__cordl_ret)
    }
    pub fn AddNamespace(
        &mut self,
        prefix: *mut crate::System::String,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNamespace", (prefix, uri))?;
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
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NoNamespaceManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlTextReaderImpl_NoNamespaceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NodeData")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextReaderImpl_NodeData {
    __cordl_parent: crate::System::Object,
    pub _cordl_type: crate::System::Xml::XmlNodeType,
    pub localName: *mut crate::System::String,
    pub prefix: *mut crate::System::String,
    pub ns: *mut crate::System::String,
    pub nameWPrefix: *mut crate::System::String,
    pub value: *mut crate::System::String,
    pub chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub valueStartPos: i32,
    pub valueLength: i32,
    pub lineInfo: crate::System::Xml::LineInfo,
    pub lineInfo2: crate::System::Xml::LineInfo,
    pub quoteChar: char,
    pub depth: i32,
    pub isEmptyOrDefault: bool,
    pub entityId: i32,
    pub xmlContextPushed: bool,
    pub nextAttrValueChunk: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    pub schemaType: *mut crate::System::Object,
    pub typedValue: *mut crate::System::Object,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NodeData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl_NodeData =>
    "System.Xml"."XmlTextReaderImpl/NodeData"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NodeData")]
impl std::ops::Deref for crate::System::Xml::XmlTextReaderImpl_NodeData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NodeData")]
impl std::ops::DerefMut for crate::System::Xml::XmlTextReaderImpl_NodeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NodeData")]
impl crate::System::Xml::XmlTextReaderImpl_NodeData {
    pub fn SetValue_String0(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetValue_Il2CppArray_i32_i32_1(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        startPos: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (chars, startPos, len))?;
        Ok(__cordl_ret)
    }
    pub fn OnBufferInvalidated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBufferInvalidated", ())?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo_StringBuilder0(
        &mut self,
        valueOffset: i32,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (valueOffset, sb))?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo_Il2CppArray_i32_i32_1(
        &mut self,
        valueOffset: i32,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CopyTo", (valueOffset, buffer, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn System_IComparable_CompareTo(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.IComparable.CompareTo", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn CreateNameWPrefix(
        &mut self,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("CreateNameWPrefix", (nt))?;
        Ok(__cordl_ret)
    }
    pub fn SetLineInfo2(
        &mut self,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLineInfo2", (lineNo, linePos))?;
        Ok(__cordl_ret)
    }
    pub fn get_StringValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_StringValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetNamedNode_XmlNodeType_String0(
        &mut self,
        _cordl_type: crate::System::Xml::XmlNodeType,
        localName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNamedNode", (_cordl_type, localName))?;
        Ok(__cordl_ret)
    }
    pub fn SetNamedNode_String_String1(
        &mut self,
        _cordl_type: crate::System::Xml::XmlNodeType,
        localName: *mut crate::System::String,
        prefix: *mut crate::System::String,
        nameWPrefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNamedNode", (_cordl_type, localName, prefix, nameWPrefix))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsDefaultAttribute(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsDefaultAttribute", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_LinePos(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueBuffered(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ValueBuffered", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsEmptyElement(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsEmptyElement", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetNameWPrefix(
        &mut self,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetNameWPrefix", (nt))?;
        Ok(__cordl_ret)
    }
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
    pub fn ClearName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearName", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetValueNode_String0(
        &mut self,
        _cordl_type: crate::System::Xml::XmlNodeType,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValueNode", (_cordl_type, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetValueNode_Il2CppArray_i32_i32_1(
        &mut self,
        _cordl_type: crate::System::Xml::XmlNodeType,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        startPos: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValueNode", (_cordl_type, chars, startPos, len))?;
        Ok(__cordl_ret)
    }
    pub fn SetLineInfo(
        &mut self,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLineInfo", (lineNo, linePos))?;
        Ok(__cordl_ret)
    }
    pub fn TrimSpacesInValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrimSpacesInValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LineNo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEmptyElement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmptyElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDefaultAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDefaultAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
        _cordl_type: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", (_cordl_type))?;
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
#[cfg(feature = "System+Xml+XmlTextReaderImpl+NodeData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlTextReaderImpl_NodeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+OnDefaultAttributeUseDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextReaderImpl_OnDefaultAttributeUseDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+OnDefaultAttributeUseDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::XmlTextReaderImpl_OnDefaultAttributeUseDelegate => "System.Xml"
    ."XmlTextReaderImpl/OnDefaultAttributeUseDelegate"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+OnDefaultAttributeUseDelegate")]
impl std::ops::Deref
for crate::System::Xml::XmlTextReaderImpl_OnDefaultAttributeUseDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+OnDefaultAttributeUseDelegate")]
impl std::ops::DerefMut
for crate::System::Xml::XmlTextReaderImpl_OnDefaultAttributeUseDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+OnDefaultAttributeUseDelegate")]
impl crate::System::Xml::XmlTextReaderImpl_OnDefaultAttributeUseDelegate {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        defaultAttribute: *mut crate::System::Xml::IDtdDefaultAttributeInfo,
        coreReader: *mut crate::System::Xml::XmlTextReaderImpl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (defaultAttribute, coreReader))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+OnDefaultAttributeUseDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlTextReaderImpl_OnDefaultAttributeUseDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingFunction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTextReaderImpl_ParsingFunction {
    AfterResolveEmptyEntityInContent = 18i32,
    AfterResolveEntityInContent = 17i32,
    DocumentContent = 5i32,
    ElementContent = 0i32,
    EntityReference = 13i32,
    Eof = 11i32,
    Error = 10i32,
    FragmentAttribute = 15i32,
    GoToEof = 20i32,
    InIncrementalRead = 14i32,
    InReadAttributeValue = 22i32,
    InReadContentAsBinary = 24i32,
    InReadElementContentAsBinary = 25i32,
    InReadValueChunk = 23i32,
    MoveToElementContent = 6i32,
    NoData = 1i32,
    OpenUrl = 2i32,
    PartialTextValue = 21i32,
    PopElementContext = 7i32,
    PopEmptyElementContext = 8i32,
    ReaderClosed = 12i32,
    ReportEndEntity = 16i32,
    ResetAttributesRootLevel = 9i32,
    SwitchToInteractive = 3i32,
    SwitchToInteractiveXmlDecl = 4i32,
    XmlDeclarationFragment = 19i32,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingFunction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl_ParsingFunction
    => "System.Xml"."XmlTextReaderImpl/ParsingFunction"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTextReaderImpl_ParsingMode {
    Full = 0i32,
    SkipContent = 2i32,
    SkipNode = 1i32,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl_ParsingMode =>
    "System.Xml"."XmlTextReaderImpl/ParsingMode"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlTextReaderImpl_ParsingState {
    pub chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub charPos: i32,
    pub charsUsed: i32,
    pub encoding: *mut crate::System::Text::Encoding,
    pub appendMode: bool,
    pub stream: *mut crate::System::IO::Stream,
    pub decoder: *mut crate::System::Text::Decoder,
    pub bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub bytePos: i32,
    pub bytesUsed: i32,
    pub textReader: *mut crate::System::IO::TextReader,
    pub lineNo: i32,
    pub lineStartPos: i32,
    pub baseUriStr: *mut crate::System::String,
    pub baseUri: *mut crate::System::Uri,
    pub isEof: bool,
    pub isStreamEof: bool,
    pub entity: *mut crate::System::Xml::IDtdEntityInfo,
    pub entityId: i32,
    pub eolNormalized: bool,
    pub entityResolvedManually: bool,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl_ParsingState =>
    "System.Xml"."XmlTextReaderImpl/ParsingState"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::XmlTextReaderImpl_ParsingState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingState")]
impl crate::System::Xml::XmlTextReaderImpl_ParsingState {
    pub fn Close(
        &mut self,
        closeInput: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Close",
            (closeInput),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_LinePos(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LinePos",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_LineNo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LineNo",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+XmlContext")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextReaderImpl_XmlContext {
    __cordl_parent: crate::System::Object,
    pub xmlSpace: crate::System::Xml::XmlSpace,
    pub xmlLang: *mut crate::System::String,
    pub defaultNamespace: *mut crate::System::String,
    pub previousContext: *mut crate::System::Xml::XmlTextReaderImpl_XmlContext,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+XmlContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl_XmlContext =>
    "System.Xml"."XmlTextReaderImpl/XmlContext"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl+XmlContext")]
impl std::ops::Deref for crate::System::Xml::XmlTextReaderImpl_XmlContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+XmlContext")]
impl std::ops::DerefMut for crate::System::Xml::XmlTextReaderImpl_XmlContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+XmlContext")]
impl crate::System::Xml::XmlTextReaderImpl_XmlContext {
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
    pub fn _ctor_XmlTextReaderImpl_XmlContext1(
        &mut self,
        previousContext: *mut crate::System::Xml::XmlTextReaderImpl_XmlContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (previousContext))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_XmlTextReaderImpl_XmlContext1(
        previousContext: *mut crate::System::Xml::XmlTextReaderImpl_XmlContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (previousContext))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl+XmlContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlTextReaderImpl_XmlContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTextReaderImpl {
    __cordl_parent: crate::System::Xml::XmlReader,
    pub useAsync: bool,
    pub laterInitParam: *mut crate::System::Xml::XmlTextReaderImpl_LaterInitParam,
    pub xmlCharType: crate::System::Xml::XmlCharType,
    pub ps: crate::System::Xml::XmlTextReaderImpl_ParsingState,
    pub parsingFunction: crate::System::Xml::XmlTextReaderImpl_ParsingFunction,
    pub nextParsingFunction: crate::System::Xml::XmlTextReaderImpl_ParsingFunction,
    pub nextNextParsingFunction: crate::System::Xml::XmlTextReaderImpl_ParsingFunction,
    pub nodes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    >,
    pub curNode: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    pub index: i32,
    pub curAttrIndex: i32,
    pub attrCount: i32,
    pub attrHashtable: i32,
    pub attrDuplWalkCount: i32,
    pub attrNeedNamespaceLookup: bool,
    pub fullAttrCleanup: bool,
    pub attrDuplSortingArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    >,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub nameTableFromSettings: bool,
    pub xmlResolver: *mut crate::System::Xml::XmlResolver,
    pub url: *mut crate::System::String,
    pub normalize: bool,
    pub supportNamespaces: bool,
    pub whitespaceHandling: crate::System::Xml::WhitespaceHandling,
    pub dtdProcessing: crate::System::Xml::DtdProcessing,
    pub entityHandling: crate::System::Xml::EntityHandling,
    pub ignorePIs: bool,
    pub ignoreComments: bool,
    pub checkCharacters: bool,
    pub lineNumberOffset: i32,
    pub linePositionOffset: i32,
    pub closeInput: bool,
    pub maxCharactersInDocument: i64,
    pub maxCharactersFromEntities: i64,
    pub v1Compat: bool,
    pub namespaceManager: *mut crate::System::Xml::XmlNamespaceManager,
    pub lastPrefix: *mut crate::System::String,
    pub xmlContext: *mut crate::System::Xml::XmlTextReaderImpl_XmlContext,
    pub parsingStatesStack: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Xml::XmlTextReaderImpl_ParsingState,
    >,
    pub parsingStatesStackTop: i32,
    pub reportedBaseUri: *mut crate::System::String,
    pub reportedEncoding: *mut crate::System::Text::Encoding,
    pub dtdInfo: *mut crate::System::Xml::IDtdInfo,
    pub fragmentType: crate::System::Xml::XmlNodeType,
    pub fragmentParserContext: *mut crate::System::Xml::XmlParserContext,
    pub fragment: bool,
    pub incReadDecoder: *mut crate::System::Xml::IncrementalReadDecoder,
    pub incReadState: crate::System::Xml::XmlTextReaderImpl_IncrementalReadState,
    pub incReadLineInfo: crate::System::Xml::LineInfo,
    pub incReadDepth: i32,
    pub incReadLeftStartPos: i32,
    pub incReadLeftEndPos: i32,
    pub attributeValueBaseEntityId: i32,
    pub emptyEntityInAttributeResolved: bool,
    pub validationEventHandling: *mut crate::System::Xml::IValidationEventHandling,
    pub onDefaultAttributeUse: *mut crate::System::Xml::XmlTextReaderImpl_OnDefaultAttributeUseDelegate,
    pub validatingReaderCompatFlag: bool,
    pub addDefaultAttributesAndNormalize: bool,
    pub stringBuilder: *mut crate::System::Text::StringBuilder,
    pub rootElementParsed: bool,
    pub standalone: bool,
    pub nextEntityId: i32,
    pub parsingMode: crate::System::Xml::XmlTextReaderImpl_ParsingMode,
    pub readState: crate::System::Xml::ReadState,
    pub lastEntity: *mut crate::System::Xml::IDtdEntityInfo,
    pub afterResetState: bool,
    pub documentStartBytePos: i32,
    pub readValueOffset: i32,
    pub charactersInDocument: i64,
    pub charactersFromEntities: i64,
    pub currentEntities: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Xml::IDtdEntityInfo,
        *mut crate::System::Xml::IDtdEntityInfo,
    >,
    pub disableUndeclaredEntityCheck: bool,
    pub outerReader: *mut crate::System::Xml::XmlReader,
    pub xmlResolverIsSet: bool,
    pub Xml: *mut crate::System::String,
    pub XmlNs: *mut crate::System::String,
    pub parseText_dummyTask: *mut crate::System::Threading::Tasks::Task_1<
        *mut crate::System::Tuple_4<i32, i32, i32, bool>,
    >,
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlTextReaderImpl => "System.Xml"
    ."XmlTextReaderImpl"
);
#[cfg(feature = "System+Xml+XmlTextReaderImpl")]
impl std::ops::Deref for crate::System::Xml::XmlTextReaderImpl {
    type Target = crate::System::Xml::XmlReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl")]
impl std::ops::DerefMut for crate::System::Xml::XmlTextReaderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl")]
impl crate::System::Xml::XmlTextReaderImpl {
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+EntityType")]
    pub type EntityType = crate::System::Xml::XmlTextReaderImpl_EntityType;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingFunction")]
    pub type ParsingFunction = crate::System::Xml::XmlTextReaderImpl_ParsingFunction;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+LaterInitParam")]
    pub type LaterInitParam = crate::System::Xml::XmlTextReaderImpl_LaterInitParam;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+InitInputType")]
    pub type InitInputType = crate::System::Xml::XmlTextReaderImpl_InitInputType;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+NoNamespaceManager")]
    pub type NoNamespaceManager = crate::System::Xml::XmlTextReaderImpl_NoNamespaceManager;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+DtdParserProxy")]
    pub type DtdParserProxy = crate::System::Xml::XmlTextReaderImpl_DtdParserProxy;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+EntityExpandType")]
    pub type EntityExpandType = crate::System::Xml::XmlTextReaderImpl_EntityExpandType;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingMode")]
    pub type ParsingMode = crate::System::Xml::XmlTextReaderImpl_ParsingMode;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+IncrementalReadState")]
    pub type IncrementalReadState = crate::System::Xml::XmlTextReaderImpl_IncrementalReadState;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+OnDefaultAttributeUseDelegate")]
    pub type OnDefaultAttributeUseDelegate = crate::System::Xml::XmlTextReaderImpl_OnDefaultAttributeUseDelegate;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+XmlContext")]
    pub type XmlContext = crate::System::Xml::XmlTextReaderImpl_XmlContext;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+ParsingState")]
    pub type ParsingState = crate::System::Xml::XmlTextReaderImpl_ParsingState;
    #[cfg(feature = "System+Xml+XmlTextReaderImpl+NodeData")]
    pub type NodeData = crate::System::Xml::XmlTextReaderImpl_NodeData;
    #[cfg(
        feature = "System+Xml+XmlTextReaderImpl+DtdDefaultAttributeInfoToNodeDataComparer"
    )]
    pub type DtdDefaultAttributeInfoToNodeDataComparer = crate::System::Xml::XmlTextReaderImpl_DtdDefaultAttributeInfoToNodeDataComparer;
    pub fn ReadValueChunk(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReadValueChunk", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn get_AttributeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AttributeCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsResolverNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsResolverNull", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTempResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlResolver> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlResolver = __cordl_object
            .invoke("GetTempResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn PushParsingState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushParsingState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_Normalization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_DtdParserProxy_Normalization", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_ParsingBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<char> = __cordl_object
            .invoke("get_DtdParserProxy_ParsingBuffer", ())?;
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
    pub fn SetErrorState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetErrorState", ())?;
        Ok(__cordl_ret)
    }
    pub fn OpenUrl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenUrl", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseNamedCharRefInline(
        &mut self,
        startPos: i32,
        expand: bool,
        internalSubsetBuilder: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "ParseNamedCharRefInline",
                (startPos, expand, internalSubsetBuilder),
            )?;
        Ok(__cordl_ret)
    }
    pub fn OnEof(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEof", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddNode(
        &mut self,
        nodeIndex: i32,
        nodeDepth: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlTextReaderImpl_NodeData = __cordl_object
            .invoke("AddNode", (nodeIndex, nodeDepth))?;
        Ok(__cordl_ret)
    }
    pub fn AddDefaultAttributesAndNormalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDefaultAttributesAndNormalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn SkipDtd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SkipDtd", ())?;
        Ok(__cordl_ret)
    }
    pub fn ThrowUnexpectedToken_i32_String0(
        &mut self,
        pos: i32,
        expectedToken: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowUnexpectedToken", (pos, expectedToken))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowUnexpectedToken_String1(
        &mut self,
        expectedToken1: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowUnexpectedToken", (expectedToken1))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowUnexpectedToken_i32_String_String2(
        &mut self,
        pos: i32,
        expectedToken1: *mut crate::System::String,
        expectedToken2: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowUnexpectedToken", (pos, expectedToken1, expectedToken2))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowUnexpectedToken_String_String3(
        &mut self,
        expectedToken1: *mut crate::System::String,
        expectedToken2: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowUnexpectedToken", (expectedToken1, expectedToken2))?;
        Ok(__cordl_ret)
    }
    pub fn GetIndexOfAttributeWithPrefix(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetIndexOfAttributeWithPrefix", (name))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToNextContentNode(
        &mut self,
        moveIfOnContentNode: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToNextContentNode", (moveIfOnContentNode))?;
        Ok(__cordl_ret)
    }
    pub fn SetDtdInfo(
        &mut self,
        newDtdInfo: *mut crate::System::Xml::IDtdInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDtdInfo", (newDtdInfo))?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_OnPublicId(
        &mut self,
        publicId: *mut crate::System::String,
        keywordLineInfo: crate::System::Xml::LineInfo,
        publicLiteralLineInfo: crate::System::Xml::LineInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DtdParserProxy_OnPublicId",
                (publicId, keywordLineInfo, publicLiteralLineInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ParsePI_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParsePI", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParsePI_StringBuilder1(
        &mut self,
        piInDtdStringBuilder: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParsePI", (piInDtdStringBuilder))?;
        Ok(__cordl_ret)
    }
    pub fn ParseEntityName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ParseEntityName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CanResolveEntity(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanResolveEntity", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ValidationEventHandling(
        &mut self,
        value: *mut crate::System::Xml::IValidationEventHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ValidationEventHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetNamespacesInScope(
        &mut self,
        scope: crate::System::Xml::XmlNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("GetNamespacesInScope", (scope))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowWithoutLineInfo_String0(
        &mut self,
        res: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowWithoutLineInfo", (res))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowWithoutLineInfo_String1(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowWithoutLineInfo", (res, arg))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowWithoutLineInfo_Il2CppArray_Exception2(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowWithoutLineInfo", (res, args, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn ShiftBuffer(
        &mut self,
        sourcePos: i32,
        destPos: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShiftBuffer", (sourcePos, destPos, count))?;
        Ok(__cordl_ret)
    }
    pub fn ParseElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishReadValueChunk(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishReadValueChunk", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CanReadValueChunk(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanReadValueChunk", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlLang(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_XmlLang", ())?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_ParsePI(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DtdParserProxy_ParsePI", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_Throw(
        &mut self,
        e: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DtdParserProxy_Throw", (e))?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_OnSystemId(
        &mut self,
        systemId: *mut crate::System::String,
        keywordLineInfo: crate::System::Xml::LineInfo,
        systemLiteralLineInfo: crate::System::Xml::LineInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DtdParserProxy_OnSystemId",
                (systemId, keywordLineInfo, systemLiteralLineInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ParseXmlDeclaration(
        &mut self,
        isTextDecl: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParseXmlDeclaration", (isTextDecl))?;
        Ok(__cordl_ret)
    }
    pub fn ParseAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn AttributeDuplCheck(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttributeDuplCheck", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsResolverSet(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsResolverSet", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDefaultNamespaceDecl(
        &mut self,
        attr: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDefaultNamespaceDecl", (attr))?;
        Ok(__cordl_ret)
    }
    pub fn OnXmlReservedAttribute(
        &mut self,
        attr: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnXmlReservedAttribute", (attr))?;
        Ok(__cordl_ret)
    }
    pub fn SkipPartialTextValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SkipPartialTextValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishInit", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupEncoding(
        &mut self,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupEncoding", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn ParseEntityReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseEntityReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDefault(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolveEntity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveEntity", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseDocumentContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseDocumentContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_EntityStackLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_DtdParserProxy_EntityStackLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEmptyElement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmptyElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishOtherValueIterator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishOtherValueIterator", ())?;
        Ok(__cordl_ret)
    }
    pub fn SkipUntil(
        &mut self,
        stopChar: char,
        recognizeLiterals: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SkipUntil", (stopChar, recognizeLiterals))?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_V1CompatibilityMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_DtdParserProxy_V1CompatibilityMode", ())?;
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
    pub fn MoveToElement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitStringInput(
        &mut self,
        baseUriStr: *mut crate::System::String,
        originalEncoding: *mut crate::System::Text::Encoding,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitStringInput", (baseUriStr, originalEncoding, str))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextNodeType(
        &mut self,
        orChars: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("GetTextNodeType", (orChars))?;
        Ok(__cordl_ret)
    }
    pub fn PushExternalEntityOrSubset(
        &mut self,
        publicId: *mut crate::System::String,
        systemId: *mut crate::System::String,
        baseUri: *mut crate::System::Uri,
        entityName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PushExternalEntityOrSubset",
                (publicId, systemId, baseUri, entityName),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ParseEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseEndElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn RegisterEntity(
        &mut self,
        entity: *mut crate::System::Xml::IDtdEntityInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterEntity", (entity))?;
        Ok(__cordl_ret)
    }
    pub fn Close_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn Close__cordl_bool1(
        &mut self,
        closeInput: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", (closeInput))?;
        Ok(__cordl_ret)
    }
    pub fn get_Depth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Depth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_CurrentPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_DtdParserProxy_CurrentPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn ThrowTagMismatch(
        &mut self,
        startTag: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowTagMismatch", (startTag))?;
        Ok(__cordl_ret)
    }
    pub fn ParseCData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseCData", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishIncrementalRead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishIncrementalRead", ())?;
        Ok(__cordl_ret)
    }
    pub fn Skip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Skip", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_DtdParserProxy_CurrentPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DtdParserProxy_CurrentPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OpenUrlDelegate(
        &mut self,
        xmlResolver: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenUrlDelegate", (xmlResolver))?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_ParsingBufferLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_DtdParserProxy_ParsingBufferLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddAttributeChunkToList(
        &mut self,
        attr: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
        chunk: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
        lastChunk: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttributeChunkToList", (attr, chunk, lastChunk))?;
        Ok(__cordl_ret)
    }
    pub fn ParseAttributeValueSlow(
        &mut self,
        curPos: i32,
        quoteChar: char,
        attr: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseAttributeValueSlow", (curPos, quoteChar, attr))?;
        Ok(__cordl_ret)
    }
    pub fn ParseRootLevelWhitespace(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseRootLevelWhitespace", ())?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_PushExternalSubset(
        &mut self,
        systemId: *mut crate::System::String,
        publicId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("DtdParserProxy_PushExternalSubset", (systemId, publicId))?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdValidation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_DtdValidation", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadDataInName(
        &mut self,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadDataInName", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn GetIndexOfAttributeWithoutPrefix(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetIndexOfAttributeWithoutPrefix", (name))?;
        Ok(__cordl_ret)
    }
    pub fn ParseDtd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseDtd", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_InternalTypedValue(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InternalTypedValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_InEntity(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_InEntity", ())?;
        Ok(__cordl_ret)
    }
    pub fn SkipPublicOrSystemIdLiteral(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SkipPublicOrSystemIdLiteral", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StandAlone(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_StandAlone", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_V1Compat(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_V1Compat", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddDefaultAttributeDtd(
        &mut self,
        defAttrInfo: *mut crate::System::Xml::IDtdDefaultAttributeInfo,
        definedInDtd: bool,
        nameSortedNodeData: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "AddDefaultAttributeDtd",
                (defAttrInfo, definedInDtd, nameSortedNodeData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetChars(
        &mut self,
        maxCharsCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetChars", (maxCharsCount))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToAttribute_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToAttribute", (name))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ParseFragmentAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseFragmentAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn PushExternalEntity(
        &mut self,
        entity: *mut crate::System::Xml::IDtdEntityInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PushExternalEntity", (entity))?;
        Ok(__cordl_ret)
    }
    pub fn AddDefaultAttributeNonDtd(
        &mut self,
        attrDef: *mut crate::System::Xml::Schema::SchemaAttDef,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddDefaultAttributeNonDtd", (attrDef))?;
        Ok(__cordl_ret)
    }
    pub fn set_DisableUndeclaredEntityCheck(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DisableUndeclaredEntityCheck", (value))?;
        Ok(__cordl_ret)
    }
    pub fn InitStreamInput_Stream_Encoding0(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitStreamInput", (stream, encoding))?;
        Ok(__cordl_ret)
    }
    pub fn InitStreamInput_String_Stream_Encoding1(
        &mut self,
        baseUriStr: *mut crate::System::String,
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitStreamInput", (baseUriStr, stream, encoding))?;
        Ok(__cordl_ret)
    }
    pub fn InitStreamInput_Uri_Stream_Encoding2(
        &mut self,
        baseUri: *mut crate::System::Uri,
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitStreamInput", (baseUri, stream, encoding))?;
        Ok(__cordl_ret)
    }
    pub fn InitStreamInput_Uri_String_Stream_Encoding3(
        &mut self,
        baseUri: *mut crate::System::Uri,
        baseUriStr: *mut crate::System::String,
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitStreamInput", (baseUri, baseUriStr, stream, encoding))?;
        Ok(__cordl_ret)
    }
    pub fn InitStreamInput_Uri_String_Stream_Il2CppArray_i32_Encoding4(
        &mut self,
        baseUri: *mut crate::System::Uri,
        baseUriStr: *mut crate::System::String,
        stream: *mut crate::System::IO::Stream,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        byteCount: i32,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitStreamInput",
                (baseUri, baseUriStr, stream, bytes, byteCount, encoding),
            )?;
        Ok(__cordl_ret)
    }
    pub fn OpenAndPush(
        &mut self,
        uri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("OpenAndPush", (uri))?;
        Ok(__cordl_ret)
    }
    pub fn get_QuoteChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("get_QuoteChar", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlResolver> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlResolver = __cordl_object
            .invoke("GetResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleEntityEnd(
        &mut self,
        checkEntityNesting: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HandleEntityEnd", (checkEntityNesting))?;
        Ok(__cordl_ret)
    }
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FragmentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("get_FragmentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn RegisterConsumedCharacters(
        &mut self,
        characters: i64,
        inEntityReference: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterConsumedCharacters", (characters, inEntityReference))?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_PushInternalDtd(
        &mut self,
        baseUri: *mut crate::System::String,
        internalDtd: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DtdParserProxy_PushInternalDtd", (baseUri, internalDtd))?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_PushEntity(
        &mut self,
        entity: *mut crate::System::Xml::IDtdEntityInfo,
        entityId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("DtdParserProxy_PushEntity", (entity, entityId))?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_NamespaceResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::IXmlNamespaceResolver> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IXmlNamespaceResolver = __cordl_object
            .invoke("get_DtdParserProxy_NamespaceResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn ElementNamespaceLookup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ElementNamespaceLookup", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishInitStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishInitStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddAttribute_i32_i32_0(
        &mut self,
        endNamePos: i32,
        colonPos: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlTextReaderImpl_NodeData = __cordl_object
            .invoke("AddAttribute", (endNamePos, colonPos))?;
        Ok(__cordl_ret)
    }
    pub fn AddAttribute_String_String_String1(
        &mut self,
        localName: *mut crate::System::String,
        prefix: *mut crate::System::String,
        nameWPrefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlTextReaderImpl_NodeData = __cordl_object
            .invoke("AddAttribute", (localName, prefix, nameWPrefix))?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_Namespaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_DtdParserProxy_Namespaces", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IXmlNamespaceResolver_LookupPrefix(
        &mut self,
        namespaceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.LookupPrefix", (namespaceName))?;
        Ok(__cordl_ret)
    }
    pub fn FinishInitUriString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishInitUriString", ())?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_ParseComment(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DtdParserProxy_ParseComment", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_LineNo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_DtdParserProxy_LineNo", ())?;
        Ok(__cordl_ret)
    }
    pub fn ThrowInvalidChar(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        length: i32,
        invCharPos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowInvalidChar", (data, length, invCharPos))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String_String_i32_i32_0(
        &mut self,
        severity: crate::System::Xml::Schema::XmlSeverityType,
        code: *mut crate::System::String,
        arg: *mut crate::System::String,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (severity, code, arg, lineNo, linePos))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_XmlSchemaException1(
        &mut self,
        severity: crate::System::Xml::Schema::XmlSeverityType,
        exception: *mut crate::System::Xml::Schema::XmlSchemaException,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (severity, exception))?;
        Ok(__cordl_ret)
    }
    pub fn CheckEncoding(
        &mut self,
        newEncodingName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoding = __cordl_object
            .invoke("CheckEncoding", (newEncodingName))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetAttribute_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAttribute", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttribute_String_String1(
        &mut self,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAttribute", (localName, namespaceURI))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttribute_i32_2(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAttribute", (i))?;
        Ok(__cordl_ret)
    }
    pub fn ReadData(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadData", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveOffEntityReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveOffEntityReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseName(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ParseName", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseComment(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseComment", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseQName_ByRefMut0(
        &mut self,
        colonPos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ParseQName", (colonPos))?;
        Ok(__cordl_ret)
    }
    pub fn ParseQName__cordl_bool_i32_ByRefMut1(
        &mut self,
        isQName: bool,
        startOffset: i32,
        colonPos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ParseQName", (isQName, startOffset, colonPos))?;
        Ok(__cordl_ret)
    }
    pub fn ZeroEndingStream(&mut self, pos: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ZeroEndingStream", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn AddDefaultAttributeInternal(
        &mut self,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
        prefix: *mut crate::System::String,
        value: *mut crate::System::String,
        lineNo: i32,
        linePos: i32,
        valueLineNo: i32,
        valueLinePos: i32,
        isXmlAttribute: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlTextReaderImpl_NodeData = __cordl_object
            .invoke(
                "AddDefaultAttributeInternal",
                (
                    localName,
                    ns,
                    prefix,
                    value,
                    lineNo,
                    linePos,
                    valueLineNo,
                    valueLinePos,
                    isXmlAttribute,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InvalidCharRecovery(
        &mut self,
        bytesCount: quest_hook::libil2cpp::ByRefMut<i32>,
        charsCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvalidCharRecovery", (bytesCount, charsCount))?;
        Ok(__cordl_ret)
    }
    pub fn ParseNumericCharRef(
        &mut self,
        expand: bool,
        internalSubsetBuilder: *mut crate::System::Text::StringBuilder,
        entityType: quest_hook::libil2cpp::ByRefMut<
            crate::System::Xml::XmlTextReaderImpl_EntityType,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ParseNumericCharRef", (expand, internalSubsetBuilder, entityType))?;
        Ok(__cordl_ret)
    }
    pub fn AllocNode(
        &mut self,
        nodeIndex: i32,
        nodeDepth: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlTextReaderImpl_NodeData = __cordl_object
            .invoke("AllocNode", (nodeIndex, nodeDepth))?;
        Ok(__cordl_ret)
    }
    pub fn FinishPartialValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishPartialValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn ThrowUnclosedElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowUnclosedElements", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseCDataOrComment_XmlNodeType0(
        &mut self,
        _cordl_type: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseCDataOrComment", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ParseCDataOrComment_ByRefMut_ByRefMut1(
        &mut self,
        _cordl_type: crate::System::Xml::XmlNodeType,
        outStartPos: quest_hook::libil2cpp::ByRefMut<i32>,
        outEndPos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParseCDataOrComment", (_cordl_type, outStartPos, outEndPos))?;
        Ok(__cordl_ret)
    }
    pub fn set_OuterReader(
        &mut self,
        value: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_OuterReader", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Settings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlReaderSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlReaderSettings = __cordl_object
            .invoke("get_Settings", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleGeneralEntityReference(
        &mut self,
        name: *mut crate::System::String,
        isInAttributeValue: bool,
        pushFakeEntityIfNullResolver: bool,
        entityStartLinePos: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::XmlTextReaderImpl_EntityType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlTextReaderImpl_EntityType = __cordl_object
            .invoke(
                "HandleGeneralEntityReference",
                (
                    name,
                    isInAttributeValue,
                    pushFakeEntityIfNullResolver,
                    entityStartLinePos,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ParseDoctypeDecl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseDoctypeDecl", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseXmlDeclarationFragment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseXmlDeclarationFragment", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitFragmentReader(
        &mut self,
        fragmentType: crate::System::Xml::XmlNodeType,
        parserContext: *mut crate::System::Xml::XmlParserContext,
        allowXmlDeclFragment: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitFragmentReader",
                (fragmentType, parserContext, allowXmlDeclFragment),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ParsePIValue(
        &mut self,
        outStartPos: quest_hook::libil2cpp::ByRefMut<i32>,
        outEndPos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParsePIValue", (outStartPos, outEndPos))?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_ReadData(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("DtdParserProxy_ReadData", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseNumericCharRefInline(
        &mut self,
        startPos: i32,
        expand: bool,
        internalSubsetBuilder: *mut crate::System::Text::StringBuilder,
        charCount: quest_hook::libil2cpp::ByRefMut<i32>,
        entityType: quest_hook::libil2cpp::ByRefMut<
            crate::System::Xml::XmlTextReaderImpl_EntityType,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "ParseNumericCharRefInline",
                (startPos, expand, internalSubsetBuilder, charCount, entityType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ParseAttributeValueChunk(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseAttributeValueChunk", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddAttributeNoChecks(
        &mut self,
        name: *mut crate::System::String,
        attrDepth: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlTextReaderImpl_NodeData = __cordl_object
            .invoke("AddAttributeNoChecks", (name, attrDepth))?;
        Ok(__cordl_ret)
    }
    pub fn get_NodeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("get_NodeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessDtdFromParserContext(
        &mut self,
        context: *mut crate::System::Xml::XmlParserContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDtdFromParserContext", (context))?;
        Ok(__cordl_ret)
    }
    pub fn PushXmlContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushXmlContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::IDtdInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IDtdInfo = __cordl_object
            .invoke("get_DtdInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleEntityReference(
        &mut self,
        isInAttributeValue: bool,
        expandType: crate::System::Xml::XmlTextReaderImpl_EntityExpandType,
        charRefEndPos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::XmlTextReaderImpl_EntityType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlTextReaderImpl_EntityType = __cordl_object
            .invoke(
                "HandleEntityReference",
                (isInAttributeValue, expandType, charRefEndPos),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_ReadState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::ReadState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::ReadState = __cordl_object
            .invoke("get_ReadState", ())?;
        Ok(__cordl_ret)
    }
    pub fn LookupNamespace_String0(
        &mut self,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn LookupNamespace_XmlTextReaderImpl_NodeData1(
        &mut self,
        node: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupNamespace", (node))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IXmlNamespaceResolver_LookupNamespace(
        &mut self,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.LookupNamespace", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn get_Namespaces(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Namespaces", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddNamespace(
        &mut self,
        prefix: *mut crate::System::String,
        uri: *mut crate::System::String,
        attr: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNamespace", (prefix, uri, attr))?;
        Ok(__cordl_ret)
    }
    pub fn GetWhitespaceType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlNodeType = __cordl_object
            .invoke("GetWhitespaceType", ())?;
        Ok(__cordl_ret)
    }
    pub fn PushInternalEntity(
        &mut self,
        entity: *mut crate::System::Xml::IDtdEntityInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushInternalEntity", (entity))?;
        Ok(__cordl_ret)
    }
    pub fn PopEntity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopEntity", ())?;
        Ok(__cordl_ret)
    }
    pub fn IncrementalRead(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IncrementalRead", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseUnexpectedToken_i32_0(
        &mut self,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ParseUnexpectedToken", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn ParseUnexpectedToken_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ParseUnexpectedToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveToNextAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToNextAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnDecodeChars(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnDecodeChars", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_OnDefaultAttributeUse(
        &mut self,
        value: *mut crate::System::Xml::XmlTextReaderImpl_OnDefaultAttributeUseDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_OnDefaultAttributeUse", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_DtdValidation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_DtdParserProxy_DtdValidation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LocalName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_InternalSchemaType(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InternalSchemaType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Normalization(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Normalization", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InternalTypedValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_InternalTypedValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_PopEntity(
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
            .invoke("DtdParserProxy_PopEntity", (oldEntity, newEntityId))?;
        Ok(__cordl_ret)
    }
    pub fn get_NamespaceManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNamespaceManager> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNamespaceManager = __cordl_object
            .invoke("get_NamespaceManager", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlNameTable0(
        &mut self,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nt))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlResolver_XmlReaderSettings_XmlParserContext1(
        &mut self,
        resolver: *mut crate::System::Xml::XmlResolver,
        settings: *mut crate::System::Xml::XmlReaderSettings,
        context: *mut crate::System::Xml::XmlParserContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resolver, settings, context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream2(
        &mut self,
        input: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Stream_XmlNameTable3(
        &mut self,
        url: *mut crate::System::String,
        input: *mut crate::System::IO::Stream,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url, input, nt))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TextReader4(
        &mut self,
        input: *mut crate::System::IO::TextReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TextReader_XmlNameTable5(
        &mut self,
        input: *mut crate::System::IO::TextReader,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input, nt))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_TextReader_XmlNameTable6(
        &mut self,
        url: *mut crate::System::String,
        input: *mut crate::System::IO::TextReader,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (url, input, nt))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_XmlNodeType_XmlParserContext7(
        &mut self,
        xmlFragment: *mut crate::System::String,
        fragType: crate::System::Xml::XmlNodeType,
        context: *mut crate::System::Xml::XmlParserContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlFragment, fragType, context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_XmlParserContext8(
        &mut self,
        xmlFragment: *mut crate::System::String,
        context: *mut crate::System::Xml::XmlParserContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlFragment, context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_Il2CppArray_i32_XmlReaderSettings_Uri_String_XmlParserContext__cordl_bool9(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        byteCount: i32,
        settings: *mut crate::System::Xml::XmlReaderSettings,
        baseUri: *mut crate::System::Uri,
        baseUriStr: *mut crate::System::String,
        context: *mut crate::System::Xml::XmlParserContext,
        closeInput: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    stream,
                    bytes,
                    byteCount,
                    settings,
                    baseUri,
                    baseUriStr,
                    context,
                    closeInput,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TextReader_XmlReaderSettings_String_XmlParserContext10(
        &mut self,
        input: *mut crate::System::IO::TextReader,
        settings: *mut crate::System::Xml::XmlReaderSettings,
        baseUriStr: *mut crate::System::String,
        context: *mut crate::System::Xml::XmlParserContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input, settings, baseUriStr, context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_XmlParserContext_XmlReaderSettings11(
        &mut self,
        xmlFragment: *mut crate::System::String,
        context: *mut crate::System::Xml::XmlParserContext,
        settings: *mut crate::System::Xml::XmlReaderSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlFragment, context, settings))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ParseElementContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseElementContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnNamespaceDecl(
        &mut self,
        attr: *mut crate::System::Xml::XmlTextReaderImpl_NodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNamespaceDecl", (attr))?;
        Ok(__cordl_ret)
    }
    pub fn DetectEncoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoding = __cordl_object
            .invoke("DetectEncoding", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasLineInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasLineInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NamespaceURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_NamespaceURI", ())?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_ParseNumericCharRef(
        &mut self,
        internalSubsetBuilder: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("DtdParserProxy_ParseNumericCharRef", (internalSubsetBuilder))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowExpectingWhitespace(
        &mut self,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowExpectingWhitespace", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InAttributeValueIterator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_InAttributeValueIterator", ())?;
        Ok(__cordl_ret)
    }
    pub fn AttributeNamespaceLookup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttributeNamespaceLookup", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_IXmlNamespaceResolver_GetNamespacesInScope(
        &mut self,
        scope: crate::System::Xml::XmlNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object
            .invoke("System.Xml.IXmlNamespaceResolver.GetNamespacesInScope", (scope))?;
        Ok(__cordl_ret)
    }
    pub fn ParseCharRefInline(
        &mut self,
        startPos: i32,
        charCount: quest_hook::libil2cpp::ByRefMut<i32>,
        entityType: quest_hook::libil2cpp::ByRefMut<
            crate::System::Xml::XmlTextReaderImpl_EntityType,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ParseCharRefInline", (startPos, charCount, entityType))?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlSpace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlSpace = __cordl_object
            .invoke("get_XmlSpace", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Prefix", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveToFirstAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToFirstAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_OnNewLine(
        &mut self,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DtdParserProxy_OnNewLine", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeCurrentNodeType(
        &mut self,
        newNodeType: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeCurrentNodeType", (newNodeType))?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_NameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlNameTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlNameTable = __cordl_object
            .invoke("get_DtdParserProxy_NameTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopParsingState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopParsingState", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupEndEntityNodeInAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupEndEntityNodeInAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseDtdFromParserContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseDtdFromParserContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_ValidationEventHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::IValidationEventHandling,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IValidationEventHandling = __cordl_object
            .invoke("get_DtdParserProxy_ValidationEventHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishInitTextReader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishInitTextReader", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReThrow(
        &mut self,
        e: *mut crate::System::Exception,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReThrow", (e, lineNo, linePos))?;
        Ok(__cordl_ret)
    }
    pub fn InitTextReaderInput_TextReader0(
        &mut self,
        baseUriStr: *mut crate::System::String,
        input: *mut crate::System::IO::TextReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitTextReaderInput", (baseUriStr, input))?;
        Ok(__cordl_ret)
    }
    pub fn InitTextReaderInput_Uri_TextReader1(
        &mut self,
        baseUriStr: *mut crate::System::String,
        baseUri: *mut crate::System::Uri,
        input: *mut crate::System::IO::TextReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitTextReaderInput", (baseUriStr, baseUri, input))?;
        Ok(__cordl_ret)
    }
    pub fn FinishReadElementContentAsBinary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishReadElementContentAsBinary", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_IsEntityEolNormalized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_DtdParserProxy_IsEntityEolNormalized", ())?;
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
    pub fn get_DtdParserProxy_LineStartPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_DtdParserProxy_LineStartPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn Throw_i32_String_String0(
        &mut self,
        pos: i32,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (pos, res, arg))?;
        Ok(__cordl_ret)
    }
    pub fn Throw_i32_String_Il2CppArray1(
        &mut self,
        pos: i32,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (pos, res, args))?;
        Ok(__cordl_ret)
    }
    pub fn Throw_i32_String2(
        &mut self,
        pos: i32,
        res: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (pos, res))?;
        Ok(__cordl_ret)
    }
    pub fn Throw_String3(
        &mut self,
        res: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (res))?;
        Ok(__cordl_ret)
    }
    pub fn Throw_String_i32_i32_4(
        &mut self,
        res: *mut crate::System::String,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (res, lineNo, linePos))?;
        Ok(__cordl_ret)
    }
    pub fn Throw_String_String5(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (res, arg))?;
        Ok(__cordl_ret)
    }
    pub fn Throw_String_String_i32_i32_6(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (res, arg, lineNo, linePos))?;
        Ok(__cordl_ret)
    }
    pub fn Throw_String_Il2CppArray7(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (res, args))?;
        Ok(__cordl_ret)
    }
    pub fn Throw_String_String_Exception8(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (res, arg, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn Throw_String_Il2CppArray_Exception9(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (res, args, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn Throw_Exception10(
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
    pub fn get_DtdParserProxy_BaseUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("get_DtdParserProxy_BaseUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseText_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ParseText", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseText_ByRefMut_ByRefMut_ByRefMut1(
        &mut self,
        startPos: quest_hook::libil2cpp::ByRefMut<i32>,
        endPos: quest_hook::libil2cpp::ByRefMut<i32>,
        outOrChars: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParseText", (startPos, endPos, outOrChars))?;
        Ok(__cordl_ret)
    }
    pub fn FinishAttributeValueIterator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishAttributeValueIterator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EOF(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EOF", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupEndEntityNodeInContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupEndEntityNodeInContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_XmlResolver(
        &mut self,
        value: *mut crate::System::Xml::XmlResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_XmlResolver", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SwitchEncoding(
        &mut self,
        newEncoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchEncoding", (newEncoding))?;
        Ok(__cordl_ret)
    }
    pub fn ReadString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BaseURI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_BaseURI", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePosition", ())?;
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
    pub fn UnregisterEntity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterEntity", ())?;
        Ok(__cordl_ret)
    }
    pub fn SwitchEncodingToUTF8(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchEncodingToUTF8", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAttributeValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadAttributeValue", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_DtdParserProxy_IsEof(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_DtdParserProxy_IsEof", ())?;
        Ok(__cordl_ret)
    }
    pub fn DtdParserProxy_ParseNamedCharRef(
        &mut self,
        expand: bool,
        internalSubsetBuilder: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "DtdParserProxy_ParseNamedCharRef",
                (expand, internalSubsetBuilder),
            )?;
        Ok(__cordl_ret)
    }
    pub fn FinishReadContentAsBinary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishReadContentAsBinary", ())?;
        Ok(__cordl_ret)
    }
    pub fn LookupPrefix(
        &mut self,
        namespaceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupPrefix", (namespaceName))?;
        Ok(__cordl_ret)
    }
    pub fn EatWhitespaces(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EatWhitespaces", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn PopElementContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopElementContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopXmlContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopXmlContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn FullAttributeCleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FullAttributeCleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupFromParserContext(
        &mut self,
        context: *mut crate::System::Xml::XmlParserContext,
        settings: *mut crate::System::Xml::XmlReaderSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupFromParserContext", (context, settings))?;
        Ok(__cordl_ret)
    }
    pub fn UriEqual(
        &mut self,
        uri1: *mut crate::System::Uri,
        uri1Str: *mut crate::System::String,
        uri2Str: *mut crate::System::String,
        resolver: *mut crate::System::Xml::XmlResolver,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UriEqual", (uri1, uri1Str, uri2Str, resolver))?;
        Ok(__cordl_ret)
    }
    pub fn New_XmlNameTable0(
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nt))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlResolver_XmlReaderSettings_XmlParserContext1(
        resolver: *mut crate::System::Xml::XmlResolver,
        settings: *mut crate::System::Xml::XmlReaderSettings,
        context: *mut crate::System::Xml::XmlParserContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resolver, settings, context))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream2(
        input: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Stream_XmlNameTable3(
        url: *mut crate::System::String,
        input: *mut crate::System::IO::Stream,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url, input, nt))?;
        Ok(__cordl_object)
    }
    pub fn New_TextReader4(
        input: *mut crate::System::IO::TextReader,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object)
    }
    pub fn New_TextReader_XmlNameTable5(
        input: *mut crate::System::IO::TextReader,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, nt))?;
        Ok(__cordl_object)
    }
    pub fn New_String_TextReader_XmlNameTable6(
        url: *mut crate::System::String,
        input: *mut crate::System::IO::TextReader,
        nt: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (url, input, nt))?;
        Ok(__cordl_object)
    }
    pub fn New_String_XmlNodeType_XmlParserContext7(
        xmlFragment: *mut crate::System::String,
        fragType: crate::System::Xml::XmlNodeType,
        context: *mut crate::System::Xml::XmlParserContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlFragment, fragType, context))?;
        Ok(__cordl_object)
    }
    pub fn New_String_XmlParserContext8(
        xmlFragment: *mut crate::System::String,
        context: *mut crate::System::Xml::XmlParserContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlFragment, context))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_Il2CppArray_i32_XmlReaderSettings_Uri_String_XmlParserContext__cordl_bool9(
        stream: *mut crate::System::IO::Stream,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        byteCount: i32,
        settings: *mut crate::System::Xml::XmlReaderSettings,
        baseUri: *mut crate::System::Uri,
        baseUriStr: *mut crate::System::String,
        context: *mut crate::System::Xml::XmlParserContext,
        closeInput: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    stream,
                    bytes,
                    byteCount,
                    settings,
                    baseUri,
                    baseUriStr,
                    context,
                    closeInput,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_TextReader_XmlReaderSettings_String_XmlParserContext10(
        input: *mut crate::System::IO::TextReader,
        settings: *mut crate::System::Xml::XmlReaderSettings,
        baseUriStr: *mut crate::System::String,
        context: *mut crate::System::Xml::XmlParserContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, settings, baseUriStr, context))?;
        Ok(__cordl_object)
    }
    pub fn New_String_XmlParserContext_XmlReaderSettings11(
        xmlFragment: *mut crate::System::String,
        context: *mut crate::System::Xml::XmlParserContext,
        settings: *mut crate::System::Xml::XmlReaderSettings,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlFragment, context, settings))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+XmlTextReaderImpl")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlTextReaderImpl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
