#[cfg(feature = "System+Xml+Schema+XmlSchemaInclude")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaInclude {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaExternal,
    pub annotation: *mut crate::System::Xml::Schema::XmlSchemaAnnotation,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInclude")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaInclude =>
    "System.Xml.Schema"."XmlSchemaInclude"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaInclude")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaInclude {
    type Target = crate::System::Xml::Schema::XmlSchemaExternal;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInclude")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaInclude {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInclude")]
impl crate::System::Xml::Schema::XmlSchemaInclude {
    pub fn AddAnnotation(
        &mut self,
        annotation: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnnotation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAnnotation", (annotation))?;
        Ok(__cordl_ret.into())
    }
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
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInclude")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XmlSchemaInclude {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
