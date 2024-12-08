#[cfg(feature = "System+Xml+Serialization+XmlChoiceIdentifierAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlChoiceIdentifierAttribute {
    __cordl_parent: crate::System::Attribute,
    pub memberName: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Serialization+XmlChoiceIdentifierAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlChoiceIdentifierAttribute =>
    "System.Xml.Serialization"."XmlChoiceIdentifierAttribute"
);
#[cfg(feature = "System+Xml+Serialization+XmlChoiceIdentifierAttribute")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlChoiceIdentifierAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlChoiceIdentifierAttribute")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlChoiceIdentifierAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlChoiceIdentifierAttribute")]
impl crate::System::Xml::Serialization::XmlChoiceIdentifierAttribute {
    pub fn get_MemberName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_MemberName", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddKeyHash(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKeyHash", (sb))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlChoiceIdentifierAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlChoiceIdentifierAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
