#[cfg(feature = "System+Xml+DtdParser")]
#[repr(C)]
#[derive(Debug)]
pub struct DtdParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub readerAdapter: quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdParserAdapter>,
    pub readerAdapterWithValidation: quest_hook::libil2cpp::Gc<
        crate::System::Xml::IDtdParserAdapterWithValidation,
    >,
    pub nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
    pub schemaInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaInfo>,
    pub xmlCharType: crate::System::Xml::XmlCharType,
    pub systemId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub publicId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub normalize: bool,
    pub validate: bool,
    pub supportNamespaces: bool,
    pub v1Compat: bool,
    pub chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub charsUsed: i32,
    pub curPos: i32,
    pub scanningFunction: crate::System::Xml::DtdParser_ScanningFunction,
    pub nextScaningFunction: crate::System::Xml::DtdParser_ScanningFunction,
    pub savedScanningFunction: crate::System::Xml::DtdParser_ScanningFunction,
    pub whitespaceSeen: bool,
    pub tokenStartPos: i32,
    pub colonPos: i32,
    pub internalSubsetValueSb: quest_hook::libil2cpp::Gc<
        crate::System::Text::StringBuilder,
    >,
    pub externalEntitiesDepth: i32,
    pub currentEntityId: i32,
    pub freeFloatingDtd: bool,
    pub hasFreeFloatingInternalSubset: bool,
    pub stringBuilder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub condSectionDepth: i32,
    pub literalLineInfo: crate::System::Xml::LineInfo,
    pub literalQuoteChar: char,
    pub documentBaseUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub externalDtdBaseUri: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub undeclaredNotations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::System::Xml::DtdParser_UndeclaredNotation>,
        >,
    >,
    pub condSectionEntityIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
}
#[cfg(feature = "System+Xml+DtdParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::DtdParser => "System.Xml"
    ."DtdParser"
);
#[cfg(feature = "System+Xml+DtdParser")]
impl std::ops::Deref for crate::System::Xml::DtdParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+DtdParser")]
impl std::ops::DerefMut for crate::System::Xml::DtdParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+DtdParser")]
impl crate::System::Xml::DtdParser {
    #[cfg(feature = "System+Xml+DtdParser+LiteralType")]
    pub type LiteralType = crate::System::Xml::DtdParser_LiteralType;
    #[cfg(feature = "System+Xml+DtdParser+ParseElementOnlyContent_LocalFrame")]
    pub type ParseElementOnlyContent_LocalFrame = crate::System::Xml::DtdParser_ParseElementOnlyContent_LocalFrame;
    #[cfg(feature = "System+Xml+DtdParser+ScanningFunction")]
    pub type ScanningFunction = crate::System::Xml::DtdParser_ScanningFunction;
    #[cfg(feature = "System+Xml+DtdParser+Token")]
    pub type Token = crate::System::Xml::DtdParser_Token;
    #[cfg(feature = "System+Xml+DtdParser+UndeclaredNotation")]
    pub type UndeclaredNotation = crate::System::Xml::DtdParser_UndeclaredNotation;
    pub fn AddUndeclaredNotation(
        &mut self,
        notationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUndeclaredNotation", (notationName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdParser>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdParser> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EatPublicKeyword(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EatPublicKeyword", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EatSystemKeyword(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EatSystemKeyword", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNameQualified(
        &mut self,
        canHavePrefix: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlQualifiedName,
        > = __cordl_object.invoke("GetNameQualified", (canHavePrefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNameString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetNameString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNmtokenString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetNmtokenString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetToken(
        &mut self,
        needWhiteSpace: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("GetToken", (needWhiteSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueWithStrippedSpaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetValueWithStrippedSpaces", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEntityEnd(
        &mut self,
        inLiteral: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HandleEntityEnd", (inLiteral))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEntityReference_XmlQualifiedName__cordl_bool1(
        &mut self,
        entityName: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        paramEntity: bool,
        inLiteral: bool,
        inAttribute: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "HandleEntityReference",
                (entityName, paramEntity, inLiteral, inAttribute),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleEntityReference__cordl_bool0(
        &mut self,
        paramEntity: bool,
        inLiteral: bool,
        inAttribute: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HandleEntityReference", (paramEntity, inLiteral, inAttribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        readerAdapter: quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdParserAdapter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (readerAdapter))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeFreeFloatingDtd(
        &mut self,
        baseUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        docTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        publicId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        systemId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        internalSubset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        adapter: quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdParserAdapter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InitializeFreeFloatingDtd",
                (baseUri, docTypeName, publicId, systemId, internalSubset, adapter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAttributeValueType(
        &mut self,
        token: crate::System::Xml::DtdParser_Token,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAttributeValueType", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadParsingBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadParsingBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnUnexpectedError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnexpectedError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        &mut self,
        saveInternalSubset: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Parse", (saveInternalSubset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseAttlistDecl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseAttlistDecl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseAttlistDefault(
        &mut self,
        attrDef: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaAttDef>,
        ignoreErrors: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseAttlistDefault", (attrDef, ignoreErrors))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseAttlistType(
        &mut self,
        attrDef: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaAttDef>,
        elementDecl: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SchemaElementDecl,
        >,
        ignoreErrors: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseAttlistType", (attrDef, elementDecl, ignoreErrors))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseComment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseComment", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseCondSection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseCondSection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseElementDecl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseElementDecl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseElementMixedContent(
        &mut self,
        pcv: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ParticleContentValidator,
        >,
        startParenEntityId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseElementMixedContent", (pcv, startParenEntityId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseElementOnlyContent(
        &mut self,
        pcv: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ParticleContentValidator,
        >,
        startParenEntityId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseElementOnlyContent", (pcv, startParenEntityId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseEntityDecl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseEntityDecl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExternalId(
        &mut self,
        idTokenType: crate::System::Xml::DtdParser_Token,
        declType: crate::System::Xml::DtdParser_Token,
        publicId: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        systemId: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseExternalId", (idTokenType, declType, publicId, systemId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExternalSubset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseExternalSubset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseFreeFloatingDtd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseFreeFloatingDtd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseHowMany(
        &mut self,
        pcv: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::ParticleContentValidator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseHowMany", (pcv))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseInDocumentDtd(
        &mut self,
        saveInternalSubset: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseInDocumentDtd", (saveInternalSubset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseInternalSubset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseInternalSubset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNotationDecl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseNotationDecl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParsePI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParsePI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSubset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseSubset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseUnexpectedToken(
        &mut self,
        startPos: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ParseUnexpectedToken", (startPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadData(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadDataInName(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadDataInName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveParsingBuffer_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveParsingBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveParsingBuffer_i32_1(
        &mut self,
        internalSubsetValueEndPos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveParsingBuffer", (internalSubsetValueEndPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanAttlist1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanAttlist1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanAttlist2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanAttlist2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanAttlist3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanAttlist3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanAttlist4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanAttlist4", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanAttlist5(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanAttlist5", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanAttlist6(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanAttlist6", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanAttlist7(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanAttlist7", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanClosingTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanClosingTag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanCondSection1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanCondSection1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanCondSection2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanCondSection2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanCondSection3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanCondSection3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanDoctype1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanDoctype1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanDoctype2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanDoctype2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanElement1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanElement1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanElement2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanElement2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanElement3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanElement3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanElement4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanElement4", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanElement5(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanElement5", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanElement6(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanElement6", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanElement7(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanElement7", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanEntity1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanEntity1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanEntity2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanEntity2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanEntity3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanEntity3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanEntityName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlQualifiedName,
        > = __cordl_object.invoke("ScanEntityName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanLiteral(
        &mut self,
        literalType: crate::System::Xml::DtdParser_LiteralType,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanLiteral", (literalType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanNameExpected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanNameExpected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanNmtoken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanNmtoken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanNmtokenExpected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanNmtokenExpected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanNotation1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanNotation1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanPublicId1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanPublicId1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanPublicId2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanPublicId2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanQNameExpected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanQNameExpected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanQName_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanQName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanQName__cordl_bool1(
        &mut self,
        isQName: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanQName", (isQName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanSubsetContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanSubsetContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanSystemId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::DtdParser_Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::DtdParser_Token = __cordl_object
            .invoke("ScanSystemId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_XmlSeverityType_Il2CppString_Il2CppString1(
        &mut self,
        severity: crate::System::Xml::Schema::XmlSeverityType,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (severity, code, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_XmlSeverityType_XmlSchemaException2(
        &mut self,
        severity: crate::System::Xml::Schema::XmlSeverityType,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (severity, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValidationEvent_i32_XmlSeverityType_Il2CppString_Il2CppString0(
        &mut self,
        pos: i32,
        severity: crate::System::Xml::Schema::XmlSeverityType,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (pos, severity, code, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn StripSpaces(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StripSpaces", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdParser_ParseFreeFloatingDtd(
        &mut self,
        baseUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        docTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        publicId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        systemId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        internalSubset: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        adapter: quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdParserAdapter>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdInfo> = __cordl_object
            .invoke(
                "System.Xml.IDtdParser.ParseFreeFloatingDtd",
                (baseUri, docTypeName, publicId, systemId, internalSubset, adapter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdParser_ParseInternalDtd(
        &mut self,
        adapter: quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdParserAdapter>,
        saveInternalSubset: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::IDtdInfo> = __cordl_object
            .invoke(
                "System.Xml.IDtdParser.ParseInternalDtd",
                (adapter, saveInternalSubset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidChar_Il2CppArray_i32_1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        length: i32,
        invCharPos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowInvalidChar", (data, length, invCharPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowInvalidChar_i32_Il2CppString0(
        &mut self,
        pos: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        invCharPos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowInvalidChar", (pos, data, invCharPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowUnexpectedToken_Il2CppString1(
        &mut self,
        pos: i32,
        expectedToken1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expectedToken2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowUnexpectedToken", (pos, expectedToken1, expectedToken2))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowUnexpectedToken_i32_Il2CppString0(
        &mut self,
        pos: i32,
        expectedToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowUnexpectedToken", (pos, expectedToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Throw_Il2CppString_i32_i32_3(
        &mut self,
        res: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (res, arg, lineNo, linePos))?;
        Ok(__cordl_ret.into())
    }
    pub fn Throw_i32_0(
        &mut self,
        curPos: i32,
        res: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (curPos, res))?;
        Ok(__cordl_ret.into())
    }
    pub fn Throw_i32_Il2CppArray2(
        &mut self,
        curPos: i32,
        res: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (curPos, res, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn Throw_i32_Il2CppString1(
        &mut self,
        curPos: i32,
        res: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Throw", (curPos, res, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyEntityReference(
        &mut self,
        entityName: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        paramEntity: bool,
        mustBeDeclared: bool,
        inAttribute: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaEntity>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SchemaEntity,
        > = __cordl_object
            .invoke(
                "VerifyEntityReference",
                (entityName, paramEntity, mustBeDeclared, inAttribute),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseUriStr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_BaseUriStr", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreEntityReferences(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IgnoreEntityReferences", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LineNo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LinePos(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Normalize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Normalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParsingInternalSubset(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ParsingInternalSubset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParsingTopLevelMarkup(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ParsingTopLevelMarkup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SaveInternalSubsetValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_SaveInternalSubsetValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SupportNamespaces(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SupportNamespaces", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+DtdParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::DtdParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+DtdParser")]
impl AsRef<crate::System::Xml::IDtdParser> for crate::System::Xml::DtdParser {
    fn as_ref(&self) -> &crate::System::Xml::IDtdParser {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+DtdParser")]
impl AsMut<crate::System::Xml::IDtdParser> for crate::System::Xml::DtdParser {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IDtdParser {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+DtdParser+LiteralType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DtdParser_LiteralType {
    #[default]
    AttributeValue = 0i32,
    EntityReplText = 1i32,
    SystemOrPublicID = 2i32,
}
#[cfg(feature = "System+Xml+DtdParser+LiteralType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::DtdParser_LiteralType =>
    "System.Xml"."DtdParser/LiteralType"
);
#[cfg(feature = "System+Xml+DtdParser+ParseElementOnlyContent_LocalFrame")]
#[repr(C)]
#[derive(Debug)]
pub struct DtdParser_ParseElementOnlyContent_LocalFrame {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub startParenEntityId: i32,
    pub parsingSchema: crate::System::Xml::DtdParser_Token,
}
#[cfg(feature = "System+Xml+DtdParser+ParseElementOnlyContent_LocalFrame")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::DtdParser_ParseElementOnlyContent_LocalFrame => "System.Xml"
    ."DtdParser/ParseElementOnlyContent_LocalFrame"
);
#[cfg(feature = "System+Xml+DtdParser+ParseElementOnlyContent_LocalFrame")]
impl std::ops::Deref
for crate::System::Xml::DtdParser_ParseElementOnlyContent_LocalFrame {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+DtdParser+ParseElementOnlyContent_LocalFrame")]
impl std::ops::DerefMut
for crate::System::Xml::DtdParser_ParseElementOnlyContent_LocalFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+DtdParser+ParseElementOnlyContent_LocalFrame")]
impl crate::System::Xml::DtdParser_ParseElementOnlyContent_LocalFrame {
    pub fn New(
        startParentEntityIdParam: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (startParentEntityIdParam))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        startParentEntityIdParam: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (startParentEntityIdParam))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+DtdParser+ParseElementOnlyContent_LocalFrame")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::DtdParser_ParseElementOnlyContent_LocalFrame {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+DtdParser+ScanningFunction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DtdParser_ScanningFunction {
    #[default]
    Attlist1 = 13i32,
    Attlist2 = 14i32,
    Attlist3 = 15i32,
    Attlist4 = 16i32,
    Attlist5 = 17i32,
    Attlist6 = 18i32,
    Attlist7 = 19i32,
    ClosingTag = 31i32,
    CondSection1 = 24i32,
    CondSection2 = 25i32,
    CondSection3 = 26i32,
    Doctype1 = 4i32,
    Doctype2 = 5i32,
    Element1 = 6i32,
    Element2 = 7i32,
    Element3 = 8i32,
    Element4 = 9i32,
    Element5 = 10i32,
    Element6 = 11i32,
    Element7 = 12i32,
    Entity1 = 20i32,
    Entity2 = 21i32,
    Entity3 = 22i32,
    Literal = 27i32,
    Name = 1i32,
    Nmtoken = 3i32,
    None = 33i32,
    Notation1 = 23i32,
    ParamEntitySpace = 32i32,
    PublicId1 = 29i32,
    PublicId2 = 30i32,
    QName = 2i32,
    SubsetContent = 0i32,
    SystemId = 28i32,
}
#[cfg(feature = "System+Xml+DtdParser+ScanningFunction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::DtdParser_ScanningFunction =>
    "System.Xml"."DtdParser/ScanningFunction"
);
#[cfg(feature = "System+Xml+DtdParser+Token")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DtdParser_Token {
    #[default]
    ANY = 44i32,
    AttlistDecl = 11i32,
    CDATA = 0i32,
    Comma = 43i32,
    Comment = 15i32,
    CondSectionEnd = 18i32,
    CondSectionStart = 17i32,
    DOCTYPE = 36i32,
    EMPTY = 45i32,
    ENTITIES = 5i32,
    ENTITY = 4i32,
    ElementDecl = 12i32,
    EntityDecl = 13i32,
    Eof = 19i32,
    FIXED = 22i32,
    GreaterThan = 29i32,
    _cordl_ID = 1i32,
    IDREF = 2i32,
    IDREFS = 3i32,
    IGNORE = 46i32,
    IMPLIED = 21i32,
    INCLUDE = 47i32,
    LeftBracket = 31i32,
    LeftParen = 27i32,
    Literal = 35i32,
    NData = 37i32,
    NMTOKEN = 6i32,
    NMTOKENS = 7i32,
    NOTATION = 8i32,
    Name = 24i32,
    Nmtoken = 25i32,
    None = 9i32,
    NotationDecl = 14i32,
    Or = 30i32,
    PCDATA = 42i32,
    PERef = 10i32,
    PI = 16i32,
    PUBLIC = 33i32,
    Percent = 38i32,
    Plus = 41i32,
    QMark = 40i32,
    QName = 23i32,
    Quote = 26i32,
    REQUIRED = 20i32,
    RightBracket = 32i32,
    RightParen = 28i32,
    SYSTEM = 34i32,
    Star = 39i32,
}
#[cfg(feature = "System+Xml+DtdParser+Token")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::DtdParser_Token => "System.Xml"
    ."DtdParser/Token"
);
#[cfg(feature = "System+Xml+DtdParser+UndeclaredNotation")]
#[repr(C)]
#[derive(Debug)]
pub struct DtdParser_UndeclaredNotation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub lineNo: i32,
    pub linePos: i32,
    pub next: quest_hook::libil2cpp::Gc<
        crate::System::Xml::DtdParser_UndeclaredNotation,
    >,
}
#[cfg(feature = "System+Xml+DtdParser+UndeclaredNotation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::DtdParser_UndeclaredNotation =>
    "System.Xml"."DtdParser/UndeclaredNotation"
);
#[cfg(feature = "System+Xml+DtdParser+UndeclaredNotation")]
impl std::ops::Deref for crate::System::Xml::DtdParser_UndeclaredNotation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+DtdParser+UndeclaredNotation")]
impl std::ops::DerefMut for crate::System::Xml::DtdParser_UndeclaredNotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+DtdParser+UndeclaredNotation")]
impl crate::System::Xml::DtdParser_UndeclaredNotation {
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, lineNo, linePos))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, lineNo, linePos))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+DtdParser+UndeclaredNotation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::DtdParser_UndeclaredNotation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
