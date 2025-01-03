#[cfg(feature = "System+Xml+Serialization+XmlIncludeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlIncludeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "System+Xml+Serialization+XmlIncludeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlIncludeAttribute
    => "System.Xml.Serialization"."XmlIncludeAttribute"
);
#[cfg(feature = "System+Xml+Serialization+XmlIncludeAttribute")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlIncludeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlIncludeAttribute")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlIncludeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlIncludeAttribute")]
impl crate::System::Xml::Serialization::XmlIncludeAttribute {
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlIncludeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlIncludeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
