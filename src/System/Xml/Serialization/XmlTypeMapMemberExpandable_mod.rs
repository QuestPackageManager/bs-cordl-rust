#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberExpandable")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTypeMapMemberExpandable {
    __cordl_parent: crate::System::Xml::Serialization::XmlTypeMapMemberElement,
    pub _flatArrayIndex: i32,
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberExpandable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlTypeMapMemberExpandable =>
    "System.Xml.Serialization"."XmlTypeMapMemberExpandable"
);
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberExpandable")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlTypeMapMemberExpandable {
    type Target = crate::System::Xml::Serialization::XmlTypeMapMemberElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberExpandable")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlTypeMapMemberExpandable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberExpandable")]
impl crate::System::Xml::Serialization::XmlTypeMapMemberExpandable {
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
    pub fn get_FlatArrayIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_FlatArrayIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FlatArrayIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FlatArrayIndex", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberExpandable")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlTypeMapMemberExpandable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
