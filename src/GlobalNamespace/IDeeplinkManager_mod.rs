#[cfg(feature = "IDeeplinkManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IDeeplinkManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IDeeplinkManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IDeeplinkManager => ""."IDeeplinkManager"
);
#[cfg(feature = "IDeeplinkManager")]
impl std::ops::Deref for IDeeplinkManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IDeeplinkManager")]
impl std::ops::DerefMut for IDeeplinkManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IDeeplinkManager")]
impl IDeeplinkManager {
    pub fn add_didReceiveDeeplinkEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut Deeplink>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didReceiveDeeplinkEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didReceiveDeeplinkEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut Deeplink>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didReceiveDeeplinkEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentDeeplink(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut Deeplink> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut Deeplink = __cordl_object
            .invoke("get_currentDeeplink", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IDeeplinkManager")]
impl quest_hook::libil2cpp::ObjectType for IDeeplinkManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
