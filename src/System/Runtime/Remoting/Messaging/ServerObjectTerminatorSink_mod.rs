#[cfg(feature = "System+Runtime+Remoting+Messaging+ServerObjectTerminatorSink")]
#[repr(C)]
#[derive(Debug)]
pub struct ServerObjectTerminatorSink {
    __cordl_parent: crate::System::Object,
    pub _nextSink: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ServerObjectTerminatorSink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::ServerObjectTerminatorSink =>
    "System.Runtime.Remoting.Messaging"."ServerObjectTerminatorSink"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+ServerObjectTerminatorSink")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::ServerObjectTerminatorSink {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ServerObjectTerminatorSink")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::ServerObjectTerminatorSink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ServerObjectTerminatorSink")]
impl crate::System::Runtime::Remoting::Messaging::ServerObjectTerminatorSink {
    pub fn AsyncProcessMessage(
        &mut self,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMessage,
        replySink: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IMessageCtrl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IMessageCtrl = __cordl_object
            .invoke("AsyncProcessMessage", (msg, replySink))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        nextSink: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nextSink))?;
        Ok(__cordl_object)
    }
    pub fn SyncProcessMessage(
        &mut self,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IMessage = __cordl_object
            .invoke("SyncProcessMessage", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        nextSink: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nextSink))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ServerObjectTerminatorSink")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::ServerObjectTerminatorSink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
