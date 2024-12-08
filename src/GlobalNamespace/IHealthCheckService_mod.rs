#[cfg(feature = "IHealthCheckService")]
#[repr(C)]
#[derive(Debug)]
pub struct IHealthCheckService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IHealthCheckService")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IHealthCheckService => ""."IHealthCheckService"
);
#[cfg(feature = "IHealthCheckService")]
impl std::ops::Deref for IHealthCheckService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl std::ops::DerefMut for IHealthCheckService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl IHealthCheckService {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl quest_hook::libil2cpp::ObjectType for IHealthCheckService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
