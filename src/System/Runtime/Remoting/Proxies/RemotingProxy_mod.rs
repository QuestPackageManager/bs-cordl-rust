#[cfg(feature = "System+Runtime+Remoting+Proxies+RemotingProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct RemotingProxy {
    __cordl_parent: crate::System::Runtime::Remoting::Proxies::RealProxy,
    pub _sink: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessageSink,
    >,
    pub _hasEnvoySink: bool,
    pub _ctorCall: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::ConstructionCall,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RemotingProxy")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Proxies::RemotingProxy {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Proxies";
    const CLASS_NAME: &'static str = "RemotingProxy";
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
#[cfg(feature = "System+Runtime+Remoting+Proxies+RemotingProxy")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Proxies::RemotingProxy {
    type Target = crate::System::Runtime::Remoting::Proxies::RealProxy;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RemotingProxy")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Proxies::RemotingProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RemotingProxy")]
impl crate::System::Runtime::Remoting::Proxies::RemotingProxy {
    pub fn ActivateRemoteObject(
        &mut self,
        request: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMethodMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = __cordl_object.invoke("ActivateRemoteObject", (request))?;
        Ok(__cordl_ret.into())
    }
    pub fn AttachIdentity(
        &mut self,
        identity: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Identity>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttachIdentity", (identity))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanCastTo(
        &mut self,
        fromType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCastTo", (fromType, o))?;
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
    pub fn Invoke(
        &mut self,
        request: quest_hook::libil2cpp::Gc<
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
        > = __cordl_object.invoke("Invoke", (request))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_ClientIdentity0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, identity))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppArray1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        activationUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        activationAttributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, activationUrl, activationAttributes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_ClientIdentity0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ClientIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, identity))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppArray1(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        activationUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        activationAttributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, activationUrl, activationAttributes))?;
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
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RemotingProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Proxies::RemotingProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RemotingProxy")]
impl AsRef<crate::System::Runtime::Remoting::IRemotingTypeInfo>
for crate::System::Runtime::Remoting::Proxies::RemotingProxy {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::IRemotingTypeInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RemotingProxy")]
impl AsMut<crate::System::Runtime::Remoting::IRemotingTypeInfo>
for crate::System::Runtime::Remoting::Proxies::RemotingProxy {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Remoting::IRemotingTypeInfo {
        unsafe { std::mem::transmute(self) }
    }
}
