#[cfg(feature = "System+Data+SqlTypes+SqlXml")]
#[repr(C)]
#[derive(Debug)]
pub struct SqlXml {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _createSqlReaderMethodInfo: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::MethodInfo,
    >,
    pub _fNotNull: bool,
    pub _stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub _firstCreateReader: bool,
}
#[cfg(feature = "System+Data+SqlTypes+SqlXml")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::SqlTypes::SqlXml {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data.SqlTypes";
    const CLASS_NAME: &'static str = "SqlXml";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlXml")]
impl std::ops::Deref for crate::System::Data::SqlTypes::SqlXml {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlXml")]
impl std::ops::DerefMut for crate::System::Data::SqlTypes::SqlXml {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlXml")]
impl crate::System::Data::SqlTypes::SqlXml {
    pub fn CreateReader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader> = __cordl_object
            .invoke("CreateReader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSqlReaderDelegate() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReaderSettings>,
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlParserContext>,
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReaderSettings>,
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlParserContext>,
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSqlReaderDelegate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSqlXmlReader(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        closeInput: bool,
        throwTargetInvocationExceptions: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateSqlXmlReader",
                (stream, closeInput, throwTargetInvocationExceptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXsdType(
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlQualifiedName,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetXsdType", (schemaSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetNull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_GetSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = __cordl_object
            .invoke("System.Xml.Serialization.IXmlSerializable.GetSchema", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_ReadXml(
        &mut self,
        r: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Xml.Serialization.IXmlSerializable.ReadXml", (r))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_WriteXml(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Xml.Serialization.IXmlSerializable.WriteXml", (writer))?;
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
    pub fn get_CreateSqlReaderMethodInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CreateSqlReaderMethodInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNull", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlXml")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::SqlTypes::SqlXml {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlXml")]
impl AsRef<crate::System::Data::SqlTypes::INullable>
for crate::System::Data::SqlTypes::SqlXml {
    fn as_ref(&self) -> &crate::System::Data::SqlTypes::INullable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlXml")]
impl AsMut<crate::System::Data::SqlTypes::INullable>
for crate::System::Data::SqlTypes::SqlXml {
    fn as_mut(&mut self) -> &mut crate::System::Data::SqlTypes::INullable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlXml")]
impl AsRef<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::SqlTypes::SqlXml {
    fn as_ref(&self) -> &crate::System::Xml::Serialization::IXmlSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SqlXml")]
impl AsMut<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::SqlTypes::SqlXml {
    fn as_mut(&mut self) -> &mut crate::System::Xml::Serialization::IXmlSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
