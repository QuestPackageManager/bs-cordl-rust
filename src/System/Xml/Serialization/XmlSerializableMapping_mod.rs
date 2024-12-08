#[cfg(feature = "System+Xml+Serialization+XmlSerializableMapping")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializableMapping {
    __cordl_parent: crate::System::Xml::Serialization::XmlTypeMapping,
    pub _schema: *mut crate::System::Xml::Schema::XmlSchema,
    pub _schemaType: *mut crate::System::Xml::Schema::XmlSchemaComplexType,
    pub _schemaTypeName: *mut crate::System::Xml::XmlQualifiedName,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializableMapping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializableMapping => "System.Xml.Serialization"
    ."XmlSerializableMapping"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializableMapping")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlSerializableMapping {
    type Target = crate::System::Xml::Serialization::XmlTypeMapping;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializableMapping")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlSerializableMapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializableMapping")]
impl crate::System::Xml::Serialization::XmlSerializableMapping {
    pub fn _ctor(
        &mut self,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        elementName: *mut crate::System::String,
        ns: *mut crate::System::String,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        xmlType: *mut crate::System::String,
        xmlTypeNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (root, elementName, ns, typeData, xmlType, xmlTypeNamespace),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        elementName: *mut crate::System::String,
        ns: *mut crate::System::String,
        typeData: *mut crate::System::Xml::Serialization::TypeData,
        xmlType: *mut crate::System::String,
        xmlTypeNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (root, elementName, ns, typeData, xmlType, xmlTypeNamespace),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializableMapping")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializableMapping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
