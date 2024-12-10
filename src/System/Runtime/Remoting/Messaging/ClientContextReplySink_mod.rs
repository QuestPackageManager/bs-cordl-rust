#[cfg(feature = "System+Runtime+Remoting+Messaging+ClientContextReplySink")]
#[repr(C)]
#[derive(Debug)]
pub struct ClientContextReplySink {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _replySink: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    pub _context: *mut crate::System::Runtime::Remoting::Contexts::Context,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ClientContextReplySink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::ClientContextReplySink =>
    "System.Runtime.Remoting.Messaging"."ClientContextReplySink"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+ClientContextReplySink")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::ClientContextReplySink {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ClientContextReplySink")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::ClientContextReplySink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ClientContextReplySink")]
impl crate::System::Runtime::Remoting::Messaging::ClientContextReplySink {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageCtrl,
        > = __cordl_object.invoke("AsyncProcessMessage", (msg, replySink))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        ctx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        >,
        replySink: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ctx, replySink))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = __cordl_object.invoke("SyncProcessMessage", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        >,
        replySink: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ctx, replySink))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ClientContextReplySink")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::ClientContextReplySink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ClientContextReplySink")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMessageSink>
for crate::System::Runtime::Remoting::Messaging::ClientContextReplySink {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Messaging::IMessageSink {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ClientContextReplySink")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMessageSink>
for crate::System::Runtime::Remoting::Messaging::ClientContextReplySink {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IMessageSink {
        unsafe { std::mem::transmute(self) }
    }
}
