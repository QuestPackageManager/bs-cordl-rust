#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAsyncCheckReaderWithLineInfoNSSchema {
    __cordl_parent: crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNS,
    pub readerAsIXmlSchemaInfo: *mut crate::System::Xml::Schema::IXmlSchemaInfo,
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema => "System.Xml"
    ."XmlAsyncCheckReaderWithLineInfoNSSchema"
);
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
impl std::ops::Deref for crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    type Target = crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNS;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
impl std::ops::DerefMut for crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
impl crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_SchemaType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaType = __cordl_object
            .invoke("System.Xml.Schema.IXmlSchemaInfo.get_SchemaType", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_IsDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.Schema.IXmlSchemaInfo.get_IsDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_IsNil(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Xml.Schema.IXmlSchemaInfo.get_IsNil", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_Validity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlSchemaValidity> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaValidity = __cordl_object
            .invoke("System.Xml.Schema.IXmlSchemaInfo.get_Validity", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_SchemaAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaAttribute = __cordl_object
            .invoke("System.Xml.Schema.IXmlSchemaInfo.get_SchemaAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_MemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaSimpleType = __cordl_object
            .invoke("System.Xml.Schema.IXmlSchemaInfo.get_MemberType", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Schema_IXmlSchemaInfo_get_SchemaElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("System.Xml.Schema.IXmlSchemaInfo.get_SchemaElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfoNSSchema")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlAsyncCheckReaderWithLineInfoNSSchema {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
