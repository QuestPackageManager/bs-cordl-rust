#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
#[repr(C)]
#[derive(Debug)]
pub struct DisposerReplySink {
    __cordl_parent: crate::System::Object,
    pub _next: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    pub _disposable: *mut crate::System::IDisposable,
}
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::DisposerReplySink =>
    "System.Runtime.Remoting"."DisposerReplySink"
);
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
impl std::ops::Deref for crate::System::Runtime::Remoting::DisposerReplySink {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::DisposerReplySink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
impl crate::System::Runtime::Remoting::DisposerReplySink {
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
        disposable: *mut crate::System::IDisposable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (next, disposable))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        next: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
        disposable: *mut crate::System::IDisposable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (next, disposable))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::DisposerReplySink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
