#[cfg(feature = "System+Xml+Schema+BaseValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseValidator {
    __cordl_parent: crate::System::Object,
    pub schemaCollection: *mut crate::System::Xml::Schema::XmlSchemaCollection,
    pub eventHandling: *mut crate::System::Xml::IValidationEventHandling,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
    pub positionInfo: *mut crate::System::Xml::PositionInfo,
    pub xmlResolver: *mut crate::System::Xml::XmlResolver,
    pub baseUri: *mut crate::System::Uri,
    pub schemaInfo: *mut crate::System::Xml::Schema::SchemaInfo,
    pub reader: *mut crate::System::Xml::XmlValidatingReaderImpl,
    pub elementName: *mut crate::System::Xml::XmlQualifiedName,
    pub context: *mut crate::System::Xml::Schema::ValidationState,
    pub textValue: *mut crate::System::Text::StringBuilder,
    pub textString: *mut crate::System::String,
    pub hasSibling: bool,
    pub checkDatatype: bool,
}
#[cfg(feature = "System+Xml+Schema+BaseValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::BaseValidator =>
    "System.Xml.Schema"."BaseValidator"
);
#[cfg(feature = "System+Xml+Schema+BaseValidator")]
impl std::ops::Deref for crate::System::Xml::Schema::BaseValidator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+BaseValidator")]
impl std::ops::DerefMut for crate::System::Xml::Schema::BaseValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+BaseValidator")]
impl crate::System::Xml::Schema::BaseValidator {
    pub fn CompleteValidation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteValidation", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindId(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("FindId", (name))?;
        Ok(__cordl_ret)
    }
    pub fn New_BaseValidator0(
        other: *mut crate::System::Xml::Schema::BaseValidator,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlValidatingReaderImpl_XmlSchemaCollection_IValidationEventHandling1(
        reader: *mut crate::System::Xml::XmlValidatingReaderImpl,
        schemaCollection: *mut crate::System::Xml::Schema::XmlSchemaCollection,
        eventHandling: *mut crate::System::Xml::IValidationEventHandling,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader, schemaCollection, eventHandling))?;
        Ok(__cordl_object)
    }
    pub fn SaveTextValue(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveTextValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String0(
        &mut self,
        code: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String_Il2CppArray1(
        &mut self,
        code: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, args))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String_Il2CppArray_XmlSeverityType5(
        &mut self,
        code: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, args, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String_String2(
        &mut self,
        code: *mut crate::System::String,
        arg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, arg))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String_String_XmlSeverityType4(
        &mut self,
        code: *mut crate::System::String,
        msg: *mut crate::System::String,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, msg, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_XmlSchemaException3(
        &mut self,
        e: *mut crate::System::Xml::Schema::XmlSchemaException,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_XmlSchemaException_XmlSeverityType6(
        &mut self,
        e: *mut crate::System::Xml::Schema::XmlSchemaException,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e, severity))?;
        Ok(__cordl_ret)
    }
    pub fn Validate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Validate", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateText", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateWhitespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateWhitespace", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BaseValidator0(
        &mut self,
        other: *mut crate::System::Xml::Schema::BaseValidator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlValidatingReaderImpl_XmlSchemaCollection_IValidationEventHandling1(
        &mut self,
        reader: *mut crate::System::Xml::XmlValidatingReaderImpl,
        schemaCollection: *mut crate::System::Xml::Schema::XmlSchemaCollection,
        eventHandling: *mut crate::System::Xml::IValidationEventHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader, schemaCollection, eventHandling))?;
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
    pub fn get_EventHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::ValidationEventHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::ValidationEventHandler = __cordl_object
            .invoke("get_EventHandler", ())?;
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
    pub fn get_PositionInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::PositionInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::PositionInfo = __cordl_object
            .invoke("get_PositionInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PreserveWhitespace(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_PreserveWhitespace", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Reader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::XmlValidatingReaderImpl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlValidatingReaderImpl = __cordl_object
            .invoke("get_Reader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaCollection = __cordl_object
            .invoke("get_SchemaCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaInfo = __cordl_object
            .invoke("get_SchemaInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::SchemaNames> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::SchemaNames = __cordl_object
            .invoke("get_SchemaNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlResolver> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlResolver = __cordl_object
            .invoke("get_XmlResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_BaseUri(
        &mut self,
        value: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BaseUri", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DtdInfo(
        &mut self,
        value: *mut crate::System::Xml::IDtdInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DtdInfo", (value))?;
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
}
#[cfg(feature = "System+Xml+Schema+BaseValidator")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::BaseValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}