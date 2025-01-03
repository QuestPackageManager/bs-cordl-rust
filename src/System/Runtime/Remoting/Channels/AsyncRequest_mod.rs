#[cfg(feature = "System+Runtime+Remoting+Channels+AsyncRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncRequest {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ReplySink: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessageSink,
    >,
    pub MsgRequest: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessage,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+AsyncRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::AsyncRequest =>
    "System.Runtime.Remoting.Channels"."AsyncRequest"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+AsyncRequest")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Channels::AsyncRequest {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+AsyncRequest")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Channels::AsyncRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+AsyncRequest")]
impl crate::System::Runtime::Remoting::Channels::AsyncRequest {
    pub fn New(
        msgRequest: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        replySink: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msgRequest, replySink))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        msgRequest: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        replySink: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msgRequest, replySink))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+AsyncRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::AsyncRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
