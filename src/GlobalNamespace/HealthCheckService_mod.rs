#[cfg(feature = "HealthCheckService")]
#[repr(C)]
#[derive(Debug)]
pub struct HealthCheckService {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _port: i32,
    pub _runThread: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
    pub _disposed: bool,
    pub _manualResetEvent: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ManualResetEvent,
    >,
    pub _listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
}
#[cfg(feature = "HealthCheckService")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HealthCheckService => ""
    ."HealthCheckService"
);
#[cfg(feature = "HealthCheckService")]
impl std::ops::Deref for crate::GlobalNamespace::HealthCheckService {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HealthCheckService")]
impl std::ops::DerefMut for crate::GlobalNamespace::HealthCheckService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HealthCheckService")]
impl crate::GlobalNamespace::HealthCheckService {
    pub const kTimeoutLengthMs: i32 = 1000i32;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (port))?;
        Ok(__cordl_object.into())
    }
    pub fn PollUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Run", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (port))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HealthCheckService")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::HealthCheckService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IHealthCheckService>>
for crate::GlobalNamespace::HealthCheckService {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IHealthCheckService> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IHealthCheckService>>
for crate::GlobalNamespace::HealthCheckService {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IHealthCheckService> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable>>
for crate::GlobalNamespace::HealthCheckService {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable>>
for crate::GlobalNamespace::HealthCheckService {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPollable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::HealthCheckService {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::GlobalNamespace::HealthCheckService {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
