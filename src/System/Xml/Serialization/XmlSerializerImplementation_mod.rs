#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializerImplementation {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializerImplementation =>
    "System.Xml.Serialization"."XmlSerializerImplementation"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlSerializerImplementation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializerImplementation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
impl crate::System::Xml::Serialization::XmlSerializerImplementation {
    pub fn get_Writer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializationWriter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializationWriter = __cordl_object
            .invoke("get_Writer", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerImplementation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializerImplementation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}