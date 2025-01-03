#[cfg(feature = "System+Xml+Schema+XmlSchemaXPath")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaXPath {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaAnnotated,
    pub xpath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaXPath")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaXPath =>
    "System.Xml.Schema"."XmlSchemaXPath"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaXPath")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaXPath {
    type Target = crate::System::Xml::Schema::XmlSchemaAnnotated;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaXPath")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaXPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaXPath")]
impl crate::System::Xml::Schema::XmlSchemaXPath {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_XPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_XPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_XPath(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_XPath", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaXPath")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XmlSchemaXPath {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
