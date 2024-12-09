#[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext")]
#[repr(C)]
#[derive(Debug)]
pub struct LogicalCallContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Datastore: *mut crate::System::Collections::Hashtable,
    pub m_RemotingData: *mut crate::System::Runtime::Remoting::Messaging::CallContextRemotingData,
    pub m_SecurityData: *mut crate::System::Runtime::Remoting::Messaging::CallContextSecurityData,
    pub m_HostContext: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_IsCorrelationMgr: bool,
    pub _sendHeaders: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Runtime::Remoting::Messaging::Header,
    >,
    pub _recvHeaders: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Runtime::Remoting::Messaging::Header,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::LogicalCallContext =>
    "System.Runtime.Remoting.Messaging"."LogicalCallContext"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::LogicalCallContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::LogicalCallContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext")]
impl crate::System::Runtime::Remoting::Messaging::LogicalCallContext {
    pub const s_CorrelationMgrSlotName: &'static str = "System.Diagnostics.Trace.CorrelationManagerSlot";
    #[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext+Reader")]
    pub type Reader = crate::System::Runtime::Remoting::Messaging::LogicalCallContext_Reader;
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetData(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetData", (name))?;
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
    pub fn Merge(
        &mut self,
        lc: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Merge", (lc))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext1(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn SetData(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        data: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (name, data))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext1(
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
    pub fn get_Datastore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::Hashtable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Hashtable = __cordl_object
            .invoke("get_Datastore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasUserData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasUserData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::LogicalCallContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext+Reader")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LogicalCallContext_Reader {
    pub m_ctx: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext+Reader")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::LogicalCallContext_Reader =>
    "System.Runtime.Remoting.Messaging"."LogicalCallContext/Reader"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext+Reader")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::Remoting::Messaging::LogicalCallContext_Reader {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+LogicalCallContext+Reader")]
impl crate::System::Runtime::Remoting::Messaging::LogicalCallContext_Reader {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    > {
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clone",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetData(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetData",
            (name),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        ctx: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ctx),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_HasInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HasInfo",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
