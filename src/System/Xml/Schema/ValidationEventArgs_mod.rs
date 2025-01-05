#[cfg(feature = "System+Xml+Schema+ValidationEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationEventArgs {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::EventArgs>,
    pub ex: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaException>,
    pub severity: crate::System::Xml::Schema::XmlSeverityType,
}
#[cfg(feature = "System+Xml+Schema+ValidationEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::ValidationEventArgs =>
    "System.Xml.Schema"."ValidationEventArgs"
);
#[cfg(feature = "System+Xml+Schema+ValidationEventArgs")]
impl std::ops::Deref for crate::System::Xml::Schema::ValidationEventArgs {
    type Target = quest_hook::libil2cpp::Gc<crate::System::EventArgs>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ValidationEventArgs")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ValidationEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ValidationEventArgs")]
impl crate::System::Xml::Schema::ValidationEventArgs {
    pub fn New_Gc0(
        ex: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ex))?;
        Ok(__cordl_object.into())
    }
    pub fn New_XmlSeverityType1(
        ex: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaException>,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ex, severity))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Gc0(
        &mut self,
        ex: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ex))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlSeverityType1(
        &mut self,
        ex: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaException>,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ex, severity))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Exception(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaException>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaException,
        > = __cordl_object.invoke("get_Exception", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Severity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlSeverityType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSeverityType = __cordl_object
            .invoke("get_Severity", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+ValidationEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::ValidationEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
