#[cfg(feature = "System+Runtime+Remoting+Messaging+StackBuilderSink")]
#[repr(C)]
#[derive(Debug)]
pub struct StackBuilderSink {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _target: *mut crate::System::MarshalByRefObject,
    pub _rp: *mut crate::System::Runtime::Remoting::Proxies::RealProxy,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+StackBuilderSink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::StackBuilderSink =>
    "System.Runtime.Remoting.Messaging"."StackBuilderSink"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+StackBuilderSink")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::StackBuilderSink {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+StackBuilderSink")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::StackBuilderSink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+StackBuilderSink")]
impl crate::System::Runtime::Remoting::Messaging::StackBuilderSink {
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
    pub fn CheckParameters(
        &mut self,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckParameters", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteAsyncMessage(
        &mut self,
        ob: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteAsyncMessage", (ob))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        obj: *mut crate::System::MarshalByRefObject,
        forceInternalExecute: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (obj, forceInternalExecute))?;
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
    pub fn _AsyncProcessMessage_b__4_0(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AsyncProcessMessage>b__4_0", (data))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        obj: *mut crate::System::MarshalByRefObject,
        forceInternalExecute: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (obj, forceInternalExecute))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+StackBuilderSink")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::StackBuilderSink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
