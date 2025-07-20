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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("Connect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "Connect", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (classToProxy, url, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn Connect_Type_Il2CppString0(
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("Connect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "Connect", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (classToProxy, url))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ClientActivatedIdentity,
                >,
                3usize,
            >("CreateClientActivatedServerIdentity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(),
                    "CreateClientActivatedServerIdentity", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientActivatedIdentity,
        > = unsafe { method.invoke_unchecked((), (realObject, objectType, objectUri))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("CreateClientProxyForContextBound")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(),
                    "CreateClientProxyForContextBound", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (_cordl_type, activationAttributes))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Remoting::ActivatedClientTypeEntry,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("CreateClientProxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "CreateClientProxy", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (entry, activationAttributes))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("CreateClientProxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "CreateClientProxy", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method.invoke_unchecked((), (objectType, url, activationAttributes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateClientProxy_WellKnownClientTypeEntry2(
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::WellKnownClientTypeEntry,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("CreateClientProxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "CreateClientProxy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (entry))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateContextBoundObjectIdentity(
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientActivatedIdentity,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ClientActivatedIdentity,
                >,
                1usize,
            >("CreateContextBoundObjectIdentity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(),
                    "CreateContextBoundObjectIdentity", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientActivatedIdentity,
        > = unsafe { method.invoke_unchecked((), (objectType))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateWellKnownServerIdentity(
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        objectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::Runtime::Remoting::WellKnownObjectMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ServerIdentity>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Runtime::Remoting::WellKnownObjectMode,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ServerIdentity,
                >,
                3usize,
            >("CreateWellKnownServerIdentity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(),
                    "CreateWellKnownServerIdentity", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ServerIdentity,
        > = unsafe { method.invoke_unchecked((), (objectType, objectUri, mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeCallData(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("DeserializeCallData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "DeserializeCallData",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisposeIdentity(
        ident: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DisposeIdentity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "DisposeIdentity", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ident))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
                3usize,
            >("FindInterfaceMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "FindInterfaceMethod",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = unsafe {
            method.invoke_unchecked((), (_cordl_type, methodName, signature))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Messaging::IMessageSink,
                >,
                3usize,
            >("GetClientChannelSinkChain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetClientChannelSinkChain",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = unsafe { method.invoke_unchecked((), (url, channelData, objectUri))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetIdentityForUri(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
                1usize,
            >("GetIdentityForUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetIdentityForUri", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Identity,
        > = unsafe { method.invoke_unchecked((), (uri))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMessageTargetIdentity(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Messaging::IMessage,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
                1usize,
            >("GetMessageTargetIdentity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetMessageTargetIdentity",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Identity,
        > = unsafe { method.invoke_unchecked((), (msg))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodBaseFromMethodMessage(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Messaging::IMethodMessage,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
                1usize,
            >("GetMethodBaseFromMethodMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetMethodBaseFromMethodMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = unsafe { method.invoke_unchecked((), (msg))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
                3usize,
            >("GetMethodBaseFromName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetMethodBaseFromName",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = unsafe {
            method.invoke_unchecked((), (_cordl_type, methodName, signature))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNormalizedUri(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetNormalizedUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetNormalizedUri", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (uri))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("GetObjectData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetObjectData", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (obj, info, context))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ClientIdentity,
                >,
                3usize,
            >("GetOrCreateClientIdentity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetOrCreateClientIdentity",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientIdentity,
        > = unsafe { method.invoke_unchecked((), (objRef, proxyType, clientProxy))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProxyForRemoteObject(
        objref: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
        classToProxy: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("GetProxyForRemoteObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetProxyForRemoteObject",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (objref, classToProxy))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRealProxy(
        proxy: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Proxies::RealProxy>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Proxies::RealProxy,
                >,
                1usize,
            >("GetRealProxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetRealProxy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Proxies::RealProxy,
        > = unsafe { method.invoke_unchecked((), (proxy))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRemoteObject(
        objRef: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
        proxyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("GetRemoteObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetRemoteObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (objRef, proxyType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetServerTypeForUri(
        URI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                1usize,
            >("GetServerTypeForUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetServerTypeForUri",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked((), (URI))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetVirtualMethod(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
                2usize,
            >("GetVirtualMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "GetVirtualMethod", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodBase,
        > = unsafe { method.invoke_unchecked((), (_cordl_type, method))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                4usize,
            >("InternalExecute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "InternalExecute", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (method, obj, parameters, out_args))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Remoting::Messaging::IMethodCallMessage,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
                >,
                2usize,
            >("InternalExecuteMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "InternalExecuteMessage",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodReturnMessage,
        > = unsafe { method.invoke_unchecked((), (target, reqMsg))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsOneWay(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>),
                bool,
                1usize,
            >("IsOneWay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "IsOneWay", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (method))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsTransparentProxy(
        proxy: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("IsTransparentProxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "IsTransparentProxy", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (proxy))? };
        Ok(__cordl_ret.into())
    }
    pub fn Marshal_Il2CppString_Type1(
        Obj: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
        ObjURI: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        RequestedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
                3usize,
            >("Marshal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "Marshal", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ObjRef,
        > = unsafe { method.invoke_unchecked((), (Obj, ObjURI, RequestedType))? };
        Ok(__cordl_ret.into())
    }
    pub fn Marshal_MarshalByRefObject0(
        Obj: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>),
                quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
                1usize,
            >("Marshal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "Marshal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ObjRef,
        > = unsafe { method.invoke_unchecked((), (Obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn NewUri() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("NewUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "NewUri", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInternalChannels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RegisterInternalChannels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "RegisterInternalChannels",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterServerIdentity(
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ServerIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::ServerIdentity,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterServerIdentity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "RegisterServerIdentity",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (identity))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAppNameFromUri(
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("RemoveAppNameFromUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "RemoveAppNameFromUri",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (uri))? };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeCallData(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("SerializeCallData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "SerializeCallData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeExceptionData(
        ex: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Exception>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("SerializeExceptionData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "SerializeExceptionData",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (ex))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetMessageTargetIdentity(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        ident: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Remoting::Messaging::IMessage,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetMessageTargetIdentity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "SetMessageTargetIdentity",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (msg, ident))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unmarshal_ObjRef0(
        objectRef: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("Unmarshal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "Unmarshal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (objectRef))? };
        Ok(__cordl_ret.into())
    }
    pub fn Unmarshal__cordl_bool1(
        objectRef: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
        fRefine: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("Unmarshal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "Unmarshal", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (objectRef, fRefine))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateOutArgObject(
        pi: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        local: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        remote: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                bool,
                3usize,
            >("UpdateOutArgObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateOutArgObject", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (pi, local, remote))?
        };
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
    const CLASS_NAME: &'static str = "RemotingServices/CACD";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Runtime::Remoting::RemotingServices_CACD as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Runtime::Remoting::RemotingServices_CACD as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
