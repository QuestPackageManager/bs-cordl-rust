#[cfg(feature = "System+Xml+IDtdAttributeListInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IDtdAttributeListInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+IDtdAttributeListInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::IDtdAttributeListInfo =>
    "System.Xml"."IDtdAttributeListInfo"
);
#[cfg(feature = "System+Xml+IDtdAttributeListInfo")]
impl std::ops::Deref for crate::System::Xml::IDtdAttributeListInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdAttributeListInfo")]
impl std::ops::DerefMut for crate::System::Xml::IDtdAttributeListInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdAttributeListInfo")]
impl crate::System::Xml::IDtdAttributeListInfo {
    pub fn LookupAttribute(
        &mut self,
        prefix: *mut quest_hook::libil2cpp::Il2CppString,
        localName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::IDtdAttributeInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::IDtdAttributeInfo = __cordl_object
            .invoke("LookupAttribute", (prefix, localName))?;
        Ok(__cordl_ret)
    }
    pub fn LookupDefaultAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Xml::IDtdDefaultAttributeInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Xml::IDtdDefaultAttributeInfo,
        > = __cordl_object.invoke("LookupDefaultAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_HasNonCDataAttributes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasNonCDataAttributes", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+IDtdAttributeListInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::IDtdAttributeListInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
