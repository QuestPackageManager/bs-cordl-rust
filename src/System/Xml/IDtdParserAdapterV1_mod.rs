#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
#[repr(C)]
#[derive(Debug)]
pub struct IDtdParserAdapterV1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::IDtdParserAdapterV1 => "System.Xml"
    ."IDtdParserAdapterV1"
);
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl std::ops::Deref for crate::System::Xml::IDtdParserAdapterV1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl std::ops::DerefMut for crate::System::Xml::IDtdParserAdapterV1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl crate::System::Xml::IDtdParserAdapterV1 {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Namespaces(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Namespaces", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Normalization(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Normalization", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_V1CompatibilityMode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_V1CompatibilityMode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
