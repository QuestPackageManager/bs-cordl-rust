#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct IWrappedCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::IWrappedCollection
    => "Newtonsoft.Json.Utilities"."IWrappedCollection"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedCollection")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::IWrappedCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedCollection")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::IWrappedCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedCollection")]
impl crate::Newtonsoft::Json::Utilities::IWrappedCollection {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_UnderlyingCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_UnderlyingCollection", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+IWrappedCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::IWrappedCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}