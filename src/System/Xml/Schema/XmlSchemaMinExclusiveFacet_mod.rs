#[cfg(feature = "System+Xml+Schema+XmlSchemaMinExclusiveFacet")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaMinExclusiveFacet {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaFacet,
    >,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaMinExclusiveFacet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaMinExclusiveFacet
    => "System.Xml.Schema"."XmlSchemaMinExclusiveFacet"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaMinExclusiveFacet")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaMinExclusiveFacet {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaFacet>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaMinExclusiveFacet")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaMinExclusiveFacet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaMinExclusiveFacet")]
impl crate::System::Xml::Schema::XmlSchemaMinExclusiveFacet {
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
#[cfg(feature = "System+Xml+Schema+XmlSchemaMinExclusiveFacet")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaMinExclusiveFacet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
