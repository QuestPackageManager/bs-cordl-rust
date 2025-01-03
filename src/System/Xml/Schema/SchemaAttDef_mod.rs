#[cfg(feature = "System+Xml+Schema+SchemaAttDef")]
#[repr(C)]
#[derive(Debug)]
pub struct SchemaAttDef {
    __cordl_parent: crate::System::Xml::Schema::SchemaDeclBase,
    pub defExpanded: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub lineNum: i32,
    pub linePos: i32,
    pub valueLineNum: i32,
    pub valueLinePos: i32,
    pub reserved: crate::System::Xml::Schema::SchemaAttDef_Reserve,
    pub defaultValueChecked: bool,
    pub schemaAttribute: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaAttribute,
    >,
}
#[cfg(feature = "System+Xml+Schema+SchemaAttDef")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SchemaAttDef =>
    "System.Xml.Schema"."SchemaAttDef"
);
#[cfg(feature = "System+Xml+Schema+SchemaAttDef")]
impl std::ops::Deref for crate::System::Xml::Schema::SchemaAttDef {
    type Target = crate::System::Xml::Schema::SchemaDeclBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaAttDef")]
impl std::ops::DerefMut for crate::System::Xml::Schema::SchemaAttDef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaAttDef")]
impl crate::System::Xml::Schema::SchemaAttDef {
    #[cfg(feature = "System+Xml+Schema+SchemaAttDef+Reserve")]
    pub type Reserve = crate::System::Xml::Schema::SchemaAttDef_Reserve;
    pub fn CheckXmlSpace(
        &mut self,
        validationEventHandling: quest_hook::libil2cpp::Gc<
            crate::System::Xml::IValidationEventHandling,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckXmlSpace", (validationEventHandling))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaAttDef>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SchemaAttDef,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_2() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_XmlQualifiedName1(
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object.into())
    }
    pub fn New_XmlQualifiedName_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, prefix))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Xml_IDtdAttributeInfo_get_IsDeclaredInExternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdAttributeInfo.get_IsDeclaredInExternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdAttributeInfo_get_IsNonCDataType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdAttributeInfo.get_IsNonCDataType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdAttributeInfo_get_IsXmlAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.IDtdAttributeInfo.get_IsXmlAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdAttributeInfo_get_LineNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Xml.IDtdAttributeInfo.get_LineNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdAttributeInfo_get_LinePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Xml.IDtdAttributeInfo.get_LinePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdAttributeInfo_get_LocalName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("System.Xml.IDtdAttributeInfo.get_LocalName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdAttributeInfo_get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("System.Xml.IDtdAttributeInfo.get_Prefix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdDefaultAttributeInfo_get_DefaultValueExpanded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("System.Xml.IDtdDefaultAttributeInfo.get_DefaultValueExpanded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdDefaultAttributeInfo_get_DefaultValueTyped(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("System.Xml.IDtdDefaultAttributeInfo.get_DefaultValueTyped", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdDefaultAttributeInfo_get_ValueLineNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Xml.IDtdDefaultAttributeInfo.get_ValueLineNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_IDtdDefaultAttributeInfo_get_ValueLinePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Xml.IDtdDefaultAttributeInfo.get_ValueLinePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlQualifiedName1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlQualifiedName_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultValueChecked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_DefaultValueChecked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultValueExpanded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DefaultValueExpanded", ())?;
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
    pub fn get_Reserved(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::SchemaAttDef_Reserve,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::SchemaAttDef_Reserve = __cordl_object
            .invoke("get_Reserved", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SchemaAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttribute,
        > = __cordl_object.invoke("get_SchemaAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TokenizedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlTokenizedType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::XmlTokenizedType = __cordl_object
            .invoke("get_TokenizedType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueLineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ValueLineNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueLinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ValueLinePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultValueExpanded(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DefaultValueExpanded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LineNumber(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LineNumber", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LinePosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LinePosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Reserved(
        &mut self,
        value: crate::System::Xml::Schema::SchemaAttDef_Reserve,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Reserved", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SchemaAttribute(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SchemaAttribute", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TokenizedType(
        &mut self,
        value: crate::System::Xml::XmlTokenizedType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TokenizedType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ValueLineNumber(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ValueLineNumber", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ValueLinePosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ValueLinePosition", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaAttDef")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::SchemaAttDef {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaAttDef")]
impl AsRef<crate::System::Xml::IDtdAttributeInfo>
for crate::System::Xml::Schema::SchemaAttDef {
    fn as_ref(&self) -> &crate::System::Xml::IDtdAttributeInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaAttDef")]
impl AsMut<crate::System::Xml::IDtdAttributeInfo>
for crate::System::Xml::Schema::SchemaAttDef {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IDtdAttributeInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaAttDef")]
impl AsRef<crate::System::Xml::IDtdDefaultAttributeInfo>
for crate::System::Xml::Schema::SchemaAttDef {
    fn as_ref(&self) -> &crate::System::Xml::IDtdDefaultAttributeInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaAttDef")]
impl AsMut<crate::System::Xml::IDtdDefaultAttributeInfo>
for crate::System::Xml::Schema::SchemaAttDef {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IDtdDefaultAttributeInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+Schema+SchemaAttDef+Reserve")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaAttDef_Reserve {
    None = 0i32,
    XmlLang = 2i32,
    XmlSpace = 1i32,
}
#[cfg(feature = "System+Xml+Schema+SchemaAttDef+Reserve")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SchemaAttDef_Reserve =>
    "System.Xml.Schema"."SchemaAttDef/Reserve"
);
