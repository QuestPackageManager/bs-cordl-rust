#[cfg(feature = "System+Runtime+Remoting+Services+ITrackingHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct ITrackingHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Services+ITrackingHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Services::ITrackingHandler =>
    "System.Runtime.Remoting.Services"."ITrackingHandler"
);
#[cfg(feature = "System+Runtime+Remoting+Services+ITrackingHandler")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Services::ITrackingHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Services+ITrackingHandler")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Services::ITrackingHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Services+ITrackingHandler")]
impl crate::System::Runtime::Remoting::Services::ITrackingHandler {
    pub fn DisconnectedObject(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectedObject", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn MarshaledObject(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        _cordl_or: *mut crate::System::Runtime::Remoting::ObjRef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarshaledObject", (obj, _cordl_or))?;
        Ok(__cordl_ret)
    }
    pub fn UnmarshaledObject(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        _cordl_or: *mut crate::System::Runtime::Remoting::ObjRef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnmarshaledObject", (obj, _cordl_or))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Services+ITrackingHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Services::ITrackingHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
