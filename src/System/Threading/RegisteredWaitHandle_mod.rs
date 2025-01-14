#[cfg(feature = "System+Threading+RegisteredWaitHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct RegisteredWaitHandle {
    __cordl_parent: crate::System::MarshalByRefObject,
    pub _waitObject: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
    pub _callback: quest_hook::libil2cpp::Gc<
        crate::System::Threading::WaitOrTimerCallback,
    >,
    pub _state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _finalEvent: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
    pub _cancelEvent: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ManualResetEvent,
    >,
    pub _timeout: crate::System::TimeSpan,
    pub _callsInProcess: i32,
    pub _executeOnlyOnce: bool,
    pub _unregistered: bool,
}
#[cfg(feature = "System+Threading+RegisteredWaitHandle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::RegisteredWaitHandle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "RegisteredWaitHandle";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DoCallBack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoCallBack", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (timedOut))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>),
                bool,
                1usize,
            >("Unregister")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Unregister", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (waitObject)) };
        Ok(__cordl_ret.into())
    }
    pub fn Wait(
        &mut self,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Wait")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Wait", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::WaitOrTimerCallback,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::TimeSpan,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (waitObject, callback, state, timeout, executeOnlyOnce),
                )
        };
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
