#[cfg(feature = "System+Runtime+Remoting+Contexts+CrossContextChannel")]
#[repr(C)]
#[derive(Debug)]
pub struct CrossContextChannel {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+CrossContextChannel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Contexts::CrossContextChannel =>
    "System.Runtime.Remoting.Contexts"."CrossContextChannel"
);
#[cfg(feature = "System+Runtime+Remoting+Contexts+CrossContextChannel")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Contexts::CrossContextChannel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+CrossContextChannel")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Contexts::CrossContextChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+CrossContextChannel")]
impl crate::System::Runtime::Remoting::Contexts::CrossContextChannel {
    #[cfg(
        feature = "System+Runtime+Remoting+Contexts+CrossContextChannel+ContextRestoreSink"
    )]
    pub type ContextRestoreSink = crate::System::Runtime::Remoting::Contexts::CrossContextChannel_ContextRestoreSink;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+CrossContextChannel")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Contexts::CrossContextChannel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+CrossContextChannel+ContextRestoreSink"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CrossContextChannel_ContextRestoreSink {
    __cordl_parent: crate::System::Object,
    pub _next: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    pub _context: *mut crate::System::Runtime::Remoting::Contexts::Context,
    pub _call: *mut crate::System::Runtime::Remoting::Messaging::IMessage,
}
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+CrossContextChannel+ContextRestoreSink"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Contexts::CrossContextChannel_ContextRestoreSink =>
    "System.Runtime.Remoting.Contexts"."CrossContextChannel/ContextRestoreSink"
);
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+CrossContextChannel+ContextRestoreSink"
)]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Contexts::CrossContextChannel_ContextRestoreSink {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+CrossContextChannel+ContextRestoreSink"
)]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Contexts::CrossContextChannel_ContextRestoreSink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+CrossContextChannel+ContextRestoreSink"
)]
impl crate::System::Runtime::Remoting::Contexts::CrossContextChannel_ContextRestoreSink {
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
        next: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
        context: *mut crate::System::Runtime::Remoting::Contexts::Context,
        call: *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (next, context, call))?;
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
        next: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
        context: *mut crate::System::Runtime::Remoting::Contexts::Context,
        call: *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (next, context, call))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+CrossContextChannel+ContextRestoreSink"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Contexts::CrossContextChannel_ContextRestoreSink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
