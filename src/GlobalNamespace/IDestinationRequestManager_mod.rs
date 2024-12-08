#[cfg(feature = "IDestinationRequestManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IDestinationRequestManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IDestinationRequestManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IDestinationRequestManager => ""
    ."IDestinationRequestManager"
);
#[cfg(feature = "IDestinationRequestManager")]
impl std::ops::Deref for IDestinationRequestManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IDestinationRequestManager")]
impl std::ops::DerefMut for IDestinationRequestManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IDestinationRequestManager")]
impl IDestinationRequestManager {
    pub fn set_currentMenuDestinationRequest(
        &mut self,
        value: *mut MenuDestination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentMenuDestinationRequest", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSendMenuDestinationRequestEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MenuDestination>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSendMenuDestinationRequestEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didSendMenuDestinationRequestEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MenuDestination>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSendMenuDestinationRequestEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentMenuDestinationRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MenuDestination> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MenuDestination = __cordl_object
            .invoke("get_currentMenuDestinationRequest", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IDestinationRequestManager")]
impl quest_hook::libil2cpp::ObjectType for IDestinationRequestManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
