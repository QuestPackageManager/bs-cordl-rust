#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
#[repr(C)]
#[derive(Debug)]
pub struct IWrappedDictionary {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::IWrappedDictionary
    => "Newtonsoft.Json.Utilities"."IWrappedDictionary"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    pub fn get_UnderlyingDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_UnderlyingDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedDictionary")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::IWrappedDictionary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
