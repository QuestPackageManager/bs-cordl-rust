#[cfg(feature = "System+Xml+Schema+BaseProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub nameTable: *mut crate::System::Xml::XmlNameTable,
    pub schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
    pub eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    pub compilationSettings: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    pub errorCount: i32,
    pub NsXml: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Xml+Schema+BaseProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::BaseProcessor =>
    "System.Xml.Schema"."BaseProcessor"
);
#[cfg(feature = "System+Xml+Schema+BaseProcessor")]
impl std::ops::Deref for crate::System::Xml::Schema::BaseProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+BaseProcessor")]
impl std::ops::DerefMut for crate::System::Xml::Schema::BaseProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+BaseProcessor")]
impl crate::System::Xml::Schema::BaseProcessor {
    pub fn AddToTable(
        &mut self,
        table: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
        qname: *mut crate::System::Xml::XmlQualifiedName,
        item: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToTable", (table, qname, item))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidAttributeGroupRedefine(
        &mut self,
        existingObject: *mut crate::System::Xml::Schema::XmlSchemaObject,
        item: *mut crate::System::Xml::Schema::XmlSchemaObject,
        table: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidAttributeGroupRedefine", (existingObject, item, table))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidGroupRedefine(
        &mut self,
        existingObject: *mut crate::System::Xml::Schema::XmlSchemaObject,
        item: *mut crate::System::Xml::Schema::XmlSchemaObject,
        table: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidGroupRedefine", (existingObject, item, table))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidTypeRedefine(
        &mut self,
        existingObject: *mut crate::System::Xml::Schema::XmlSchemaObject,
        item: *mut crate::System::Xml::Schema::XmlSchemaObject,
        table: *mut crate::System::Xml::Schema::XmlSchemaObjectTable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidTypeRedefine", (existingObject, item, table))?;
        Ok(__cordl_ret)
    }
    pub fn New_XmlNameTable_SchemaNames_ValidationEventHandler0(
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameTable, schemaNames, eventHandler))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlSchemaCompilationSettings1(
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
        compilationSettings: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (nameTable, schemaNames, eventHandler, compilationSettings),
            )?;
        Ok(__cordl_object)
    }
    pub fn SendValidationEventNoThrow(
        &mut self,
        e: *mut crate::System::Xml::Schema::XmlSchemaException,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEventNoThrow", (e, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppArray_Exception_XmlSchemaObject3(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        innerException: *mut crate::System::Exception,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, args, innerException, source))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppString_Il2CppString_Il2CppString_i32_i32_4(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        msg1: *mut quest_hook::libil2cpp::Il2CppString,
        msg2: *mut quest_hook::libil2cpp::Il2CppString,
        sourceUri: *mut quest_hook::libil2cpp::Il2CppString,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendValidationEvent",
                (code, msg1, msg2, sourceUri, lineNumber, linePosition),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppString_Il2CppString_XmlSchemaObject2(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        msg1: *mut quest_hook::libil2cpp::Il2CppString,
        msg2: *mut quest_hook::libil2cpp::Il2CppString,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, msg1, msg2, source))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppString_XmlSchemaObject1(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        msg: *mut quest_hook::libil2cpp::Il2CppString,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, msg, source))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_Il2CppString_XmlSchemaObject_XmlSeverityType7(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        msg: *mut quest_hook::libil2cpp::Il2CppString,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, msg, source, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_XmlSchemaObject0(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, source))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_Il2CppString_XmlSchemaObject_XmlSeverityType5(
        &mut self,
        code: *mut quest_hook::libil2cpp::Il2CppString,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, source, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_XmlSchemaException6(
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
    pub fn SendValidationEvent_XmlSchemaException_XmlSeverityType8(
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
    pub fn _ctor_XmlNameTable_SchemaNames_ValidationEventHandler0(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nameTable, schemaNames, eventHandler))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlSchemaCompilationSettings1(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
        compilationSettings: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (nameTable, schemaNames, eventHandler, compilationSettings),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_CompilationSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaCompilationSettings = __cordl_object
            .invoke("get_CompilationSettings", ())?;
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
    pub fn get_HasErrors(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasErrors", ())?;
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
}
#[cfg(feature = "System+Xml+Schema+BaseProcessor")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::BaseProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
