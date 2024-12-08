#[cfg(feature = "System+Runtime+Remoting+Proxies+RemotingProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct RemotingProxy {
    __cordl_parent: crate::System::Runtime::Remoting::Proxies::RealProxy,
    pub _sink: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    pub _hasEnvoySink: bool,
    pub _ctorCall: *mut crate::System::Runtime::Remoting::Messaging::ConstructionCall,
}
#[cfg(feature = "System+Runtime+Remoting+Proxies+RemotingProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Proxies::RemotingProxy =>
    "System.Runtime.Remoting.Proxies"."RemotingProxy"
);
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
        request: *mut crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IMessage = __cordl_object
            .invoke("ActivateRemoteObject", (request))?;
        Ok(__cordl_ret)
    }
    pub fn AttachIdentity(
        &mut self,
        identity: *mut crate::System::Runtime::Remoting::Identity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttachIdentity", (identity))?;
        Ok(__cordl_ret)
    }
    pub fn CanCastTo(
        &mut self,
        fromType: *mut crate::System::Type,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanCastTo", (fromType, o))?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        request: *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IMessage = __cordl_object
            .invoke("Invoke", (request))?;
        Ok(__cordl_ret)
    }
    pub fn New_ClientIdentity0(
        _cordl_type: *mut crate::System::Type,
        identity: *mut crate::System::Runtime::Remoting::ClientIdentity,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, identity))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray1(
        _cordl_type: *mut crate::System::Type,
        activationUrl: *mut crate::System::String,
        activationAttributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, activationUrl, activationAttributes))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ClientIdentity0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        identity: *mut crate::System::Runtime::Remoting::ClientIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, identity))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        activationUrl: *mut crate::System::String,
        activationAttributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, activationUrl, activationAttributes))?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_TypeName", ())?;
        Ok(__cordl_ret)
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
