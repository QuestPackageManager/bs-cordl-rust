#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstructionResponse {
    __cordl_parent: crate::System::Runtime::Remoting::Messaging::MethodResponse,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionResponse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::ConstructionResponse =>
    "System.Runtime.Remoting.Messaging"."ConstructionResponse"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionResponse")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::ConstructionResponse {
    type Target = crate::System::Runtime::Remoting::Messaging::MethodResponse;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionResponse")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::ConstructionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionResponse")]
impl crate::System::Runtime::Remoting::Messaging::ConstructionResponse {
    pub fn New_Exception_IMethodCallMessage1(
        e: *mut crate::System::Exception,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (e, msg))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppObject_LogicalCallContext_IMethodCallMessage0(
        resultObject: *mut quest_hook::libil2cpp::Il2CppObject,
        callCtx: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resultObject, callCtx, msg))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext2(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Exception_IMethodCallMessage1(
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
    pub fn _ctor_Il2CppObject_LogicalCallContext_IMethodCallMessage0(
        &mut self,
        resultObject: *mut quest_hook::libil2cpp::Il2CppObject,
        callCtx: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resultObject, callCtx, msg))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
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
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionResponse")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::ConstructionResponse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
