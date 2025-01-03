#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoMethodMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub method: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeMethodInfo>,
    pub args: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
    >,
    pub names: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub arg_types: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub ctx: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    >,
    pub rval: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub exc: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    pub asyncResult: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::AsyncResult,
    >,
    pub call_type: crate::System::Runtime::Remoting::Messaging::CallType,
    pub uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub properties: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::MCMDictionary,
    >,
    pub identity: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    pub methodSignature: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::MonoMethodMessage =>
    "System.Runtime.Remoting.Messaging"."MonoMethodMessage"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    pub fn GetArg(
        &mut self,
        arg_num: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetArg", (arg_num))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodInfo(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodInfo", (_cordl_type, methodName))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitMessage(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeMethodInfo>,
        out_args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitMessage", (method, out_args))?;
        Ok(__cordl_ret.into())
    }
    pub fn NeedsOutProcessing(
        &mut self,
        outCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("NeedsOutProcessing", (outCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_MethodBase_Il2CppArray0(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        out_args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, out_args))?;
        Ok(__cordl_object.into())
    }
    pub fn New_MethodInfo_Il2CppArray_Il2CppArray1(
        minfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        in_args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        out_args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (minfo, in_args, out_args))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type_Il2CppString_Il2CppArray2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        in_args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, methodName, in_args))?;
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
    pub fn _ctor_MethodBase_Il2CppArray0(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        out_args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method, out_args))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_MethodInfo_Il2CppArray_Il2CppArray1(
        &mut self,
        minfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        in_args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        out_args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (minfo, in_args, out_args))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type_Il2CppString_Il2CppArray2(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        in_args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, methodName, in_args))?;
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
    pub fn get_AsyncResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::AsyncResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::AsyncResult,
        > = __cordl_object.invoke("get_AsyncResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CallType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Remoting::Messaging::CallType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::Remoting::Messaging::CallType = __cordl_object
            .invoke("get_CallType", ())?;
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
    pub fn get_OutArgCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_OutArgCount", ())?;
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
    pub fn set_LogicalCallContext(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LogicalCallContext", (value))?;
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
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IInternalMessage>
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Messaging::IInternalMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IInternalMessage>
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IInternalMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMessage>
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Messaging::IMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMessage>
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Remoting::Messaging::IMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMethodCallMessage>
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Remoting::Messaging::IMethodCallMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMethodCallMessage>
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMethodMessage>
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Messaging::IMethodMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMethodMessage>
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IMethodMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage>
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MonoMethodMessage")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage>
for crate::System::Runtime::Remoting::Messaging::MonoMethodMessage {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage {
        unsafe { std::mem::transmute(self) }
    }
}
