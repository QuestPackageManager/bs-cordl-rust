#[cfg(feature = "System+Runtime+Remoting+Messaging+EnvoyTerminatorSink")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvoyTerminatorSink {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+EnvoyTerminatorSink")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Messaging::EnvoyTerminatorSink {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Messaging";
    const CLASS_NAME: &'static str = "EnvoyTerminatorSink";
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
#[cfg(feature = "System+Runtime+Remoting+Messaging+EnvoyTerminatorSink")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::EnvoyTerminatorSink {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+EnvoyTerminatorSink")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::EnvoyTerminatorSink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+EnvoyTerminatorSink")]
impl crate::System::Runtime::Remoting::Messaging::EnvoyTerminatorSink {
    pub fn AsyncProcessMessage(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        replySink: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageCtrl,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Remoting::Messaging::IMessage,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Remoting::Messaging::IMessageSink,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Messaging::IMessageCtrl,
                >,
                2usize,
            >("AsyncProcessMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AsyncProcessMessage", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageCtrl,
        > = unsafe { method.invoke_unchecked(self, (msg, replySink)) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SyncProcessMessage(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Messaging::IMessage,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Messaging::IMessage,
                >,
                1usize,
            >("SyncProcessMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SyncProcessMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = unsafe { method.invoke_unchecked(self, (msg)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+EnvoyTerminatorSink")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::EnvoyTerminatorSink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+EnvoyTerminatorSink")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMessageSink>
for crate::System::Runtime::Remoting::Messaging::EnvoyTerminatorSink {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Messaging::IMessageSink {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+EnvoyTerminatorSink")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMessageSink>
for crate::System::Runtime::Remoting::Messaging::EnvoyTerminatorSink {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IMessageSink {
        unsafe { std::mem::transmute(self) }
    }
}
