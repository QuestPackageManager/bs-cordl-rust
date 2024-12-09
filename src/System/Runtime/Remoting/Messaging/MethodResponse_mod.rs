#[cfg(feature = "System+Runtime+Remoting+Messaging+MethodResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct MethodResponse {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _methodName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _uri: *mut quest_hook::libil2cpp::Il2CppString,
    pub _typeName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _methodBase: *mut crate::System::Reflection::MethodBase,
    pub _returnValue: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _exception: *mut crate::System::Exception,
    pub _methodSignature: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Type,
    >,
    pub _inArgInfo: *mut crate::System::Runtime::Remoting::Messaging::ArgInfo,
    pub _args: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _outArgs: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _callMsg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
    pub _callContext: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    pub _targetIdentity: *mut crate::System::Runtime::Remoting::Identity,
    pub ExternalProperties: *mut crate::System::Collections::IDictionary,
    pub InternalProperties: *mut crate::System::Collections::IDictionary,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MethodResponse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::MethodResponse =>
    "System.Runtime.Remoting.Messaging"."MethodResponse"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+MethodResponse")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::MethodResponse {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MethodResponse")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Messaging::MethodResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MethodResponse")]
impl crate::System::Runtime::Remoting::Messaging::MethodResponse {
    pub fn GetArg(
        &mut self,
        argNum: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetArg", (argNum))?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn InitMethodProperty(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitMethodProperty", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn New_Exception_IMethodCallMessage0(
        e: *mut crate::System::Exception,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (e, msg))?;
        Ok(__cordl_object)
    }
    pub fn New_IMethodCallMessage_CADMethodReturnMessage2(
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
        retmsg: *mut crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msg, retmsg))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppObject_Il2CppArray_LogicalCallContext_IMethodCallMessage1(
        returnValue: *mut quest_hook::libil2cpp::Il2CppObject,
        outArgs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        callCtx: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (returnValue, outArgs, callCtx, msg))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext3(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn System_Runtime_Remoting_Messaging_IInternalMessage_get_TargetIdentity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Runtime::Remoting::Identity> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Identity = __cordl_object
            .invoke(
                "System.Runtime.Remoting.Messaging.IInternalMessage.get_TargetIdentity",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Runtime_Remoting_Messaging_IInternalMessage_get_Uri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("System.Runtime.Remoting.Messaging.IInternalMessage.get_Uri", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Runtime_Remoting_Messaging_IInternalMessage_set_TargetIdentity(
        &mut self,
        value: *mut crate::System::Runtime::Remoting::Identity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Remoting.Messaging.IInternalMessage.set_TargetIdentity",
                (value),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Runtime_Remoting_Messaging_IInternalMessage_set_Uri(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Remoting.Messaging.IInternalMessage.set_Uri",
                (value),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Exception_IMethodCallMessage0(
        &mut self,
        e: *mut crate::System::Exception,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (e, msg))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IMethodCallMessage_CADMethodReturnMessage2(
        &mut self,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
        retmsg: *mut crate::System::Runtime::Remoting::Messaging::CADMethodReturnMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msg, retmsg))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppObject_Il2CppArray_LogicalCallContext_IMethodCallMessage1(
        &mut self,
        returnValue: *mut quest_hook::libil2cpp::Il2CppObject,
        outArgs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        callCtx: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (returnValue, outArgs, callCtx, msg))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext3(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn get_ArgCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ArgCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Args(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Args", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Exception(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("get_Exception", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LogicalCallContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext = __cordl_object
            .invoke("get_LogicalCallContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MethodBase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodBase = __cordl_object
            .invoke("get_MethodBase", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MethodName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_MethodName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MethodSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_MethodSignature", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OutArgs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_OutArgs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IDictionary> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionary = __cordl_object
            .invoke("get_Properties", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReturnValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_ReturnValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_TypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Uri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Uri", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Uri(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Uri", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MethodResponse")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::MethodResponse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
