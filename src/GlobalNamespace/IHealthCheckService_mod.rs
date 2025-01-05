#[cfg(feature = "IHealthCheckService")]
#[repr(C)]
#[derive(Debug)]
pub struct IHealthCheckService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IHealthCheckService")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IHealthCheckService => ""
    ."IHealthCheckService"
);
#[cfg(feature = "IHealthCheckService")]
impl std::ops::Deref for crate::GlobalNamespace::IHealthCheckService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl std::ops::DerefMut for crate::GlobalNamespace::IHealthCheckService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl crate::GlobalNamespace::IHealthCheckService {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IHealthCheckService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IHealthCheckService")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable>>
for crate::GlobalNamespace::IHealthCheckService {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable>>
for crate::GlobalNamespace::IHealthCheckService {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::IHealthCheckService {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IHealthCheckService")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::IHealthCheckService {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
