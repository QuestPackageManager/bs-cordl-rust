#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberFlatList")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTypeMapMemberFlatList {
    __cordl_parent: crate::System::Xml::Serialization::XmlTypeMapMemberExpandable,
    pub _listMap: *mut crate::System::Xml::Serialization::ListMap,
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberFlatList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlTypeMapMemberFlatList => "System.Xml.Serialization"
    ."XmlTypeMapMemberFlatList"
);
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberFlatList")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlTypeMapMemberFlatList {
    type Target = crate::System::Xml::Serialization::XmlTypeMapMemberExpandable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberFlatList")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlTypeMapMemberFlatList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberFlatList")]
impl crate::System::Xml::Serialization::XmlTypeMapMemberFlatList {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ListMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Serialization::ListMap> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::ListMap = __cordl_object
            .invoke("get_ListMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ListMap(
        &mut self,
        value: *mut crate::System::Xml::Serialization::ListMap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ListMap", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberFlatList")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlTypeMapMemberFlatList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}