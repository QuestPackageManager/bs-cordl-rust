#[cfg(feature = "IRichPresenceData")]
#[repr(C)]
#[derive(Debug)]
pub struct IRichPresenceData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IRichPresenceData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IRichPresenceData => ""
    ."IRichPresenceData"
);
#[cfg(feature = "IRichPresenceData")]
impl std::ops::Deref for crate::GlobalNamespace::IRichPresenceData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IRichPresenceData")]
impl std::ops::DerefMut for crate::GlobalNamespace::IRichPresenceData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IRichPresenceData")]
impl crate::GlobalNamespace::IRichPresenceData {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_apiName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_apiName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localizedDescription(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_localizedDescription", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IRichPresenceData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IRichPresenceData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
