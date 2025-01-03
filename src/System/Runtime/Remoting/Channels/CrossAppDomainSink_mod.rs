#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink")]
#[repr(C)]
#[derive(Debug)]
pub struct CrossAppDomainSink {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _domainID: i32,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::CrossAppDomainSink =>
    "System.Runtime.Remoting.Channels"."CrossAppDomainSink"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink")]
impl crate::System::Runtime::Remoting::Channels::CrossAppDomainSink {
    #[cfg(
        feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink+ProcessMessageRes"
    )]
    pub type ProcessMessageRes = crate::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes;
    pub fn AsyncProcessMessage(
        &mut self,
        reqMsg: quest_hook::libil2cpp::Gc<
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
        > = __cordl_object.invoke("AsyncProcessMessage", (reqMsg, replySink))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSink(
        domainID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Channels::CrossAppDomainSink,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Channels::CrossAppDomainSink,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSink", (domainID))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        domainID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (domainID))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessMessageInDomain(
        arrRequest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        cadMsg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::CADMethodCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes,
    > {
        let __cordl_ret: crate::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessMessageInDomain", (arrRequest, cadMsg))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendAsyncMessage(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendAsyncMessage", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncProcessMessage(
        &mut self,
        msgRequest: quest_hook::libil2cpp::Gc<
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
        > = __cordl_object.invoke("SyncProcessMessage", (msgRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn _AsyncProcessMessage_b__10_0(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AsyncProcessMessage>b__10_0", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        domainID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (domainID))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TargetDomainId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TargetDomainId", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMessageSink>
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Messaging::IMessageSink {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMessageSink>
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IMessageSink {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink+ProcessMessageRes")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CrossAppDomainSink_ProcessMessageRes {
    pub arrResponse: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub cadMrm: *mut crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink+ProcessMessageRes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes =>
    "System.Runtime.Remoting.Channels"."CrossAppDomainSink/ProcessMessageRes"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink+ProcessMessageRes")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink+ProcessMessageRes")]
impl crate::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes {}
