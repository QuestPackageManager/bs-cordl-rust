#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberNamespaces")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTypeMapMemberNamespaces {
    __cordl_parent: crate::System::Xml::Serialization::XmlTypeMapMember,
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberNamespaces")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlTypeMapMemberNamespaces =>
    "System.Xml.Serialization"."XmlTypeMapMemberNamespaces"
);
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberNamespaces")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlTypeMapMemberNamespaces {
    type Target = crate::System::Xml::Serialization::XmlTypeMapMember;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberNamespaces")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlTypeMapMemberNamespaces {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberNamespaces")]
impl crate::System::Xml::Serialization::XmlTypeMapMemberNamespaces {
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
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberNamespaces")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlTypeMapMemberNamespaces {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
