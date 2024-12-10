#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct ReturnMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _outArgs: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _args: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _callCtx: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    pub _returnValue: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _uri: *mut quest_hook::libil2cpp::Il2CppString,
    pub _exception: *mut crate::System::Exception,
    pub _methodBase: *mut crate::System::Reflection::MethodBase,
    pub _methodName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _methodSignature: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Type,
    >,
    pub _typeName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _properties: *mut crate::System::Runtime::Remoting::Messaging::MethodReturnDictionary,
    pub _targetIdentity: *mut crate::System::Runtime::Remoting::Identity,
    pub _inArgInfo: *mut crate::System::Runtime::Remoting::Messaging::ArgInfo,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::ReturnMessage =>
    "System.Runtime.Remoting.Messaging"."ReturnMessage"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    pub fn GetArg(
        &mut self,
        argNum: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetArg", (argNum))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Exception_IMethodCallMessage1(
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        mcm: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (e, mcm))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject_Il2CppArray_i32_LogicalCallContext_IMethodCallMessage0(
        ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        outArgs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        outArgsCount: i32,
        callCtx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        >,
        mcm: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ret, outArgs, outArgsCount, callCtx, mcm))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Runtime_Remoting_Messaging_IInternalMessage_get_TargetIdentity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Identity,
        > = __cordl_object
            .invoke(
                "System.Runtime.Remoting.Messaging.IInternalMessage.get_TargetIdentity",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Remoting_Messaging_IInternalMessage_get_Uri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("System.Runtime.Remoting.Messaging.IInternalMessage.get_Uri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Remoting_Messaging_IInternalMessage_set_TargetIdentity(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Remoting.Messaging.IInternalMessage.set_TargetIdentity",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Remoting_Messaging_IInternalMessage_set_Uri(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Remoting.Messaging.IInternalMessage.set_Uri",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Exception_IMethodCallMessage1(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        mcm: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (e, mcm))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_Il2CppArray_i32_LogicalCallContext_IMethodCallMessage0(
        &mut self,
        ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        outArgs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        outArgsCount: i32,
        callCtx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        >,
        mcm: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ret, outArgs, outArgsCount, callCtx, mcm))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ArgCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ArgCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Args(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("get_Args", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Exception(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = __cordl_object
            .invoke("get_Exception", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LogicalCallContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        > = __cordl_object.invoke("get_LogicalCallContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MethodBase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = __cordl_object.invoke("get_MethodBase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MethodName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_MethodName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MethodSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_MethodSignature", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OutArgs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("get_OutArgs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IDictionary,
        > = __cordl_object.invoke("get_Properties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReturnValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_ReturnValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_TypeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Uri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Uri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Uri(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Uri", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IInternalMessage>
for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Messaging::IInternalMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IInternalMessage>
for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IInternalMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMessage>
for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Messaging::IMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMessage>
for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Remoting::Messaging::IMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMethodMessage>
for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Messaging::IMethodMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMethodMessage>
for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IMethodMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage>
for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ReturnMessage")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage>
for crate::System::Runtime::Remoting::Messaging::ReturnMessage {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
        unsafe { std::mem::transmute(self) }
    }
}
