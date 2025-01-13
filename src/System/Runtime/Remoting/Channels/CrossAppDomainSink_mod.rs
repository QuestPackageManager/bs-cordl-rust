#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink")]
#[repr(C)]
#[derive(Debug)]
pub struct CrossAppDomainSink {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _domainID: i32,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Channels";
    const CLASS_NAME: &'static str = "CrossAppDomainSink";
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CrossAppDomainSink_ProcessMessageRes {
    pub arrResponse: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub cadMrm: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink+ProcessMessageRes")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Channels";
    const CLASS_NAME: &'static str = "CrossAppDomainSink/ProcessMessageRes";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink+ProcessMessageRes")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink+ProcessMessageRes")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink+ProcessMessageRes")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainSink+ProcessMessageRes")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Runtime::Remoting::Channels::CrossAppDomainSink_ProcessMessageRes {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
