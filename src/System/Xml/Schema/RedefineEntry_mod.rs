#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct RedefineEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub redefine: *mut crate::System::Xml::Schema::XmlSchemaRedefine,
    pub schemaToUpdate: *mut crate::System::Xml::Schema::XmlSchema,
}
#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::RedefineEntry =>
    "System.Xml.Schema"."RedefineEntry"
);
#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
impl std::ops::Deref for crate::System::Xml::Schema::RedefineEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
impl std::ops::DerefMut for crate::System::Xml::Schema::RedefineEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
impl crate::System::Xml::Schema::RedefineEntry {
    pub fn New(
        external: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaRedefine,
        >,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (external, schema))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        external: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaRedefine,
        >,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (external, schema))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+RedefineEntry")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::RedefineEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
