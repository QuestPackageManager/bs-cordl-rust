#[cfg(feature = "HealthCheckService")]
#[repr(C)]
#[derive(Debug)]
pub struct HealthCheckService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _port: i32,
    pub _runThread: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
    pub _disposed: bool,
    pub _manualResetEvent: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ManualResetEvent,
    >,
    pub _listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
}
#[cfg(feature = "HealthCheckService")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::HealthCheckService {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "HealthCheckService";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "HealthCheckService")]
impl std::ops::Deref for crate::GlobalNamespace::HealthCheckService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::HealthCheckService as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::HealthCheckService as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::HealthCheckService as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PollUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::HealthCheckService as
                    quest_hook::libil2cpp::Type > ::class(), "PollUpdate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::HealthCheckService as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Run")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::HealthCheckService as
                    quest_hook::libil2cpp::Type > ::class(), "Run", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::HealthCheckService as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::HealthCheckService as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (port))?
        };
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
impl AsRef<crate::GlobalNamespace::IHealthCheckService>
for crate::GlobalNamespace::HealthCheckService {
    fn as_ref(&self) -> &crate::GlobalNamespace::IHealthCheckService {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsMut<crate::GlobalNamespace::IHealthCheckService>
for crate::GlobalNamespace::HealthCheckService {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IHealthCheckService {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsRef<crate::GlobalNamespace::IPollable>
for crate::GlobalNamespace::HealthCheckService {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPollable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsMut<crate::GlobalNamespace::IPollable>
for crate::GlobalNamespace::HealthCheckService {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPollable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::HealthCheckService {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HealthCheckService")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::HealthCheckService {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
