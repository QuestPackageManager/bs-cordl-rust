#[cfg(feature = "System+Runtime+Remoting+IEnvoyInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IEnvoyInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+IEnvoyInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::IEnvoyInfo =>
    "System.Runtime.Remoting"."IEnvoyInfo"
);
#[cfg(feature = "System+Runtime+Remoting+IEnvoyInfo")]
impl std::ops::Deref for crate::System::Runtime::Remoting::IEnvoyInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+IEnvoyInfo")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::IEnvoyInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+IEnvoyInfo")]
impl crate::System::Runtime::Remoting::IEnvoyInfo {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_EnvoySinks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink = __cordl_object
            .invoke("get_EnvoySinks", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+IEnvoyInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Runtime::Remoting::IEnvoyInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
