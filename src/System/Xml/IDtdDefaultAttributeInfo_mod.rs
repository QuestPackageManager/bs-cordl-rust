#[cfg(feature = "System+Xml+IDtdDefaultAttributeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IDtdDefaultAttributeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+IDtdDefaultAttributeInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::IDtdDefaultAttributeInfo =>
    "System.Xml"."IDtdDefaultAttributeInfo"
);
#[cfg(feature = "System+Xml+IDtdDefaultAttributeInfo")]
impl std::ops::Deref for crate::System::Xml::IDtdDefaultAttributeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdDefaultAttributeInfo")]
impl std::ops::DerefMut for crate::System::Xml::IDtdDefaultAttributeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdDefaultAttributeInfo")]
impl crate::System::Xml::IDtdDefaultAttributeInfo {
    pub fn get_DefaultValueExpanded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DefaultValueExpanded", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueLinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ValueLinePosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultValueTyped(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_DefaultValueTyped", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueLineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ValueLineNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Xml+IDtdDefaultAttributeInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::IDtdDefaultAttributeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
