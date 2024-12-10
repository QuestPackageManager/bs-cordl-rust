#[cfg(feature = "System+Threading+RegisteredWaitHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct RegisteredWaitHandle {
    __cordl_parent: crate::System::MarshalByRefObject,
    pub _waitObject: *mut crate::System::Threading::WaitHandle,
    pub _callback: *mut crate::System::Threading::WaitOrTimerCallback,
    pub _state: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _finalEvent: *mut crate::System::Threading::WaitHandle,
    pub _cancelEvent: *mut crate::System::Threading::ManualResetEvent,
    pub _timeout: crate::System::TimeSpan,
    pub _callsInProcess: i32,
    pub _executeOnlyOnce: bool,
    pub _unregistered: bool,
}
#[cfg(feature = "System+Threading+RegisteredWaitHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::RegisteredWaitHandle =>
    "System.Threading"."RegisteredWaitHandle"
);
#[cfg(feature = "System+Threading+RegisteredWaitHandle")]
impl std::ops::Deref for crate::System::Threading::RegisteredWaitHandle {
    type Target = crate::System::MarshalByRefObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+RegisteredWaitHandle")]
impl std::ops::DerefMut for crate::System::Threading::RegisteredWaitHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+RegisteredWaitHandle")]
impl crate::System::Threading::RegisteredWaitHandle {
    pub fn DoCallBack(
        &mut self,
        timedOut: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoCallBack", (timedOut))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        waitObject: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Threading::WaitOrTimerCallback,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        timeout: crate::System::TimeSpan,
        executeOnlyOnce: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (waitObject, callback, state, timeout, executeOnlyOnce),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Unregister(
        &mut self,
        waitObject: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Unregister", (waitObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn Wait(
        &mut self,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Wait", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        waitObject: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Threading::WaitOrTimerCallback,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        timeout: crate::System::TimeSpan,
        executeOnlyOnce: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (waitObject, callback, state, timeout, executeOnlyOnce))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+RegisteredWaitHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::RegisteredWaitHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
