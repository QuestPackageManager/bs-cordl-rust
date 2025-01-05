#[cfg(feature = "System+Runtime+Remoting+Activation+ActivationServices")]
#[repr(C)]
#[derive(Debug)]
pub struct ActivationServices {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+Remoting+Activation+ActivationServices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Activation::ActivationServices =>
    "System.Runtime.Remoting.Activation"."ActivationServices"
);
#[cfg(feature = "System+Runtime+Remoting+Activation+ActivationServices")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Activation::ActivationServices {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+ActivationServices")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Activation::ActivationServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+ActivationServices")]
impl crate::System::Runtime::Remoting::Activation::ActivationServices {
    pub fn Activate(
        proxy: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Proxies::RemotingProxy,
        >,
        ctorCall: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::ConstructionCall,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Activate", (proxy, ctorCall))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocateUninitializedClassInstance(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocateUninitializedClassInstance", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateConstructionCall(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        activationUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        activationAttributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::ConstructionCall,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::ConstructionCall,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateConstructionCall",
                (_cordl_type, activationUrl, activationAttributes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstanceFromMessage(
        ctorCall: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInstanceFromMessage", (ctorCall))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateProxyForType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateProxyForType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableProxyActivation(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableProxyActivation", (_cordl_type, enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoteActivate(
        ctorCall: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoteActivate", (ctorCall))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConstructionActivator() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ConstructionActivator", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+ActivationServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Activation::ActivationServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
