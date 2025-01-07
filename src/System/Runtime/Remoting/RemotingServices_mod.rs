#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
#[repr(C)]
#[derive(Debug)]
pub struct RemotingServices {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::RemotingServices {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting";
    const CLASS_NAME: &'static str = "RemotingServices";
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
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
impl std::ops::Deref for crate::System::Runtime::Remoting::RemotingServices {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::RemotingServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
impl crate::System::Runtime::Remoting::RemotingServices {
    #[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
    pub type CACD = crate::System::Runtime::Remoting::RemotingServices_CACD;
    pub fn Connect_Il2CppObject1(
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Connect", (classToProxy, url, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn Connect_Type_Il2CppString0(
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Connect", (classToProxy, url))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClientActivatedServerIdentity(
        realObject: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        objectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientActivatedIdentity,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientActivatedIdentity,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateClientActivatedServerIdentity",
                (realObject, objectType, objectUri),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClientProxyForContextBound(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        activationAttributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateClientProxyForContextBound",
                (_cordl_type, activationAttributes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClientProxy_ActivatedClientTypeEntry_Il2CppArray0(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ActivatedClientTypeEntry,
        >,
        activationAttributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateClientProxy", (entry, activationAttributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClientProxy_Type_Il2CppString_Il2CppArray1(
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        activationAttributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateClientProxy", (objectType, url, activationAttributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClientProxy_WellKnownClientTypeEntry2(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateClientProxy", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateContextBoundObjectIdentity(
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientActivatedIdentity,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientActivatedIdentity,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateContextBoundObjectIdentity", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateWellKnownServerIdentity(
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        objectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::Runtime::Remoting::WellKnownObjectMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ServerIdentity>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ServerIdentity,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateWellKnownServerIdentity", (objectType, objectUri, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeCallData(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeCallData", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisposeIdentity(
        ident: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisposeIdentity", (ident))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindInterfaceMethod(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindInterfaceMethod", (_cordl_type, methodName, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClientChannelSinkChain(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        channelData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        objectUri: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClientChannelSinkChain", (url, channelData, objectUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIdentityForUri(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Identity,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIdentityForUri", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMessageTargetIdentity(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Identity,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMessageTargetIdentity", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodBaseFromMethodMessage(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodBaseFromMethodMessage", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodBaseFromName(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodBaseFromName", (_cordl_type, methodName, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNormalizedUri(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNormalizedUri", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectData", (obj, info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateClientIdentity(
        objRef: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
        proxyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        clientProxy: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ClientIdentity>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientIdentity,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreateClientIdentity", (objRef, proxyType, clientProxy))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProxyForRemoteObject(
        objref: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProxyForRemoteObject", (objref, classToProxy))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRealProxy(
        proxy: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Proxies::RealProxy>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Proxies::RealProxy,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRealProxy", (proxy))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRemoteObject(
        objRef: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
        proxyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRemoteObject", (objRef, proxyType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServerTypeForUri(
        URI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetServerTypeForUri", (URI))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVirtualMethod(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVirtualMethod", (_cordl_type, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalExecute(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        out_args: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalExecute", (method, obj, parameters, out_args))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalExecuteMessage(
        target: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
        reqMsg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalExecuteMessage", (target, reqMsg))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOneWay(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOneWay", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTransparentProxy(
        proxy: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTransparentProxy", (proxy))?;
        Ok(__cordl_ret.into())
    }
    pub fn Marshal_Il2CppString_Type1(
        Obj: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
        ObjURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        RequestedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ObjRef,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Marshal", (Obj, ObjURI, RequestedType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Marshal_MarshalByRefObject0(
        Obj: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ObjRef,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Marshal", (Obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewUri() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("NewUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInternalChannels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterInternalChannels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterServerIdentity(
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ServerIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterServerIdentity", (identity))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAppNameFromUri(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveAppNameFromUri", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeCallData(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeCallData", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeExceptionData(
        ex: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeExceptionData", (ex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMessageTargetIdentity(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        ident: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMessageTargetIdentity", (msg, ident))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unmarshal_ObjRef0(
        objectRef: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unmarshal", (objectRef))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unmarshal__cordl_bool1(
        objectRef: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
        fRefine: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unmarshal", (objectRef, fRefine))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateOutArgObject(
        pi: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        local: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        remote: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateOutArgObject", (pi, local, remote))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::RemotingServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
#[repr(C)]
#[derive(Debug)]
pub struct RemotingServices_CACD {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub d: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub c: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::RemotingServices_CACD {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting";
    const CLASS_NAME: &'static str = "CACD";
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
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
impl std::ops::Deref for crate::System::Runtime::Remoting::RemotingServices_CACD {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::RemotingServices_CACD {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
impl crate::System::Runtime::Remoting::RemotingServices_CACD {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "System+Runtime+Remoting+RemotingServices+CACD")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::RemotingServices_CACD {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
