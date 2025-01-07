#[cfg(feature = "System+Xml+Serialization+XmlSerializableMapping")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializableMapping {
    __cordl_parent: crate::System::Xml::Serialization::XmlTypeMapping,
    pub _schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    pub _schemaType: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaComplexType,
    >,
    pub _schemaTypeName: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializableMapping")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Serialization::XmlSerializableMapping {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Serialization";
    const CLASS_NAME: &'static str = "XmlSerializableMapping";
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
    pub fn New(
        root: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
        elementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeData: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        xmlType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlTypeNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (root, elementName, ns, typeData, xmlType, xmlTypeNamespace),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        root: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
        elementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        typeData: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        xmlType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xmlTypeNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (root, elementName, ns, typeData, xmlType, xmlTypeNamespace),
            )?;
        Ok(__cordl_ret.into())
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
