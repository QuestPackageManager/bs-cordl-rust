#[cfg(feature = "System+Runtime+Remoting+Contexts+Context")]
#[repr(C)]
#[derive(Debug)]
pub struct Context {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub domain_id: i32,
    pub context_id: i32,
    pub static_data: crate::System::UIntPtr,
    pub data: crate::System::UIntPtr,
    pub server_context_sink_chain: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessageSink,
    >,
    pub client_context_sink_chain: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessageSink,
    >,
    pub context_properties: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Runtime::Remoting::Contexts::IContextProperty,
            >,
        >,
    >,
    pub _localDataStore: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreHolder>,
    pub context_dynamic_properties: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection,
    >,
    pub callback_object: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Contexts::ContextCallbackObject,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+Context")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Contexts::Context {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Contexts";
    const CLASS_NAME: &'static str = "Context";
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
#[cfg(feature = "System+Runtime+Remoting+Contexts+Context")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Contexts::Context {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+Context")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Contexts::Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+Context")]
impl crate::System::Runtime::Remoting::Contexts::Context {
    pub fn AllocateDataSlot() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocateDataSlot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocateNamedDataSlot(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocateNamedDataSlot", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEnvoySink(
        &mut self,
        serverObject: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = __cordl_object.invoke("CreateEnvoySink", (serverObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNewContext(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Contexts::Context>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateNewContext", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateServerObjectSinkChain(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
        forceInternalExecute: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = __cordl_object
            .invoke("CreateServerObjectSinkChain", (obj, forceInternalExecute))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoCallBack(
        &mut self,
        deleg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::CrossContextDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoCallBack", (deleg))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeNamedDataSlot(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FreeNamedDataSlot", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn Freeze(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Freeze", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClientContextSinkChain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = __cordl_object.invoke("GetClientContextSinkChain", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetData(
        slot: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetData", (slot))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDynamicPropertyCollection(
        obj: quest_hook::libil2cpp::Gc<crate::System::ContextBoundObject>,
        ctx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDynamicPropertyCollection", (obj, ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNamedDataSlot(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNamedDataSlot", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProperty(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::IContextProperty,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::IContextProperty,
        > = __cordl_object.invoke("GetProperty", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServerContextSinkChain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = __cordl_object.invoke("GetServerContextSinkChain", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyDynamicSinks(
        &mut self,
        start: bool,
        req_msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        client_site: bool,
        _cordl_async: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyDynamicSinks", (start, req_msg, client_site, _cordl_async))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyGlobalDynamicSinks(
        start: bool,
        req_msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        client_site: bool,
        _cordl_async: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "NotifyGlobalDynamicSinks",
                (start, req_msg, client_site, _cordl_async),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterContext(
        ctx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterContext", (ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDynamicProperty(
        prop: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::IDynamicProperty,
        >,
        obj: quest_hook::libil2cpp::Gc<crate::System::ContextBoundObject>,
        ctx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterDynamicProperty", (prop, obj, ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseContext(
        ctx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseContext", (ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        slot: quest_hook::libil2cpp::Gc<crate::System::LocalDataStoreSlot>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetData", (slot, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProperty(
        &mut self,
        prop: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::IContextProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetProperty", (prop))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchToContext(
        newContext: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Contexts::Context>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SwitchToContext", (newContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDynamicProperty(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        obj: quest_hook::libil2cpp::Gc<crate::System::ContextBoundObject>,
        ctx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnregisterDynamicProperty", (name, obj, ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContextID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ContextID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContextProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Contexts::IContextProperty,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Contexts::IContextProperty,
                >,
            >,
        > = __cordl_object.invoke("get_ContextProperties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Contexts::Context>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasDynamicSinks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDynamicSinks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasExitSinks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasExitSinks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasGlobalDynamicSinks() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_HasGlobalDynamicSinks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDefaultContext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDefaultContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MyLocalStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::LocalDataStore>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::LocalDataStore> = __cordl_object
            .invoke("get_MyLocalStore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NeedsContextSink(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_NeedsContextSink", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+Context")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Contexts::Context {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
