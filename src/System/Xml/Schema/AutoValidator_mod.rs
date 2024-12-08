#[cfg(feature = "System+Xml+Schema+AutoValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct AutoValidator {
    __cordl_parent: crate::System::Xml::Schema::BaseValidator,
}
#[cfg(feature = "System+Xml+Schema+AutoValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::AutoValidator =>
    "System.Xml.Schema"."AutoValidator"
);
#[cfg(feature = "System+Xml+Schema+AutoValidator")]
impl std::ops::Deref for crate::System::Xml::Schema::AutoValidator {
    type Target = crate::System::Xml::Schema::BaseValidator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+AutoValidator")]
impl std::ops::DerefMut for crate::System::Xml::Schema::AutoValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+AutoValidator")]
impl crate::System::Xml::Schema::AutoValidator {
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
    pub fn DetectValidationType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::ValidationType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::ValidationType = __cordl_object
            .invoke("DetectValidationType", ())?;
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
    pub fn New(
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
    pub fn _ctor(
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
    pub fn get_PreserveWhitespace(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_PreserveWhitespace", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+AutoValidator")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::AutoValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
