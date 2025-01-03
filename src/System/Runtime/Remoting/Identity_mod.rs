#[cfg(feature = "System+Runtime+Remoting+Identity")]
#[repr(C)]
#[derive(Debug)]
pub struct Identity {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _objectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _channelSink: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessageSink,
    >,
    pub _envoySink: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessageSink,
    >,
    pub _clientDynamicProperties: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection,
    >,
    pub _serverDynamicProperties: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection,
    >,
    pub _objRef: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    pub _disposed: bool,
}
#[cfg(feature = "System+Runtime+Remoting+Identity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::Identity =>
    "System.Runtime.Remoting"."Identity"
);
#[cfg(feature = "System+Runtime+Remoting+Identity")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Identity {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Identity")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Identity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Identity")]
impl crate::System::Runtime::Remoting::Identity {
    pub fn CreateObjRef(
        &mut self,
        requestedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::ObjRef,
        > = __cordl_object.invoke("CreateObjRef", (requestedType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        objectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectUri))?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyClientDynamicSinks(
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
            .invoke(
                "NotifyClientDynamicSinks",
                (start, req_msg, client_site, _cordl_async),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyServerDynamicSinks(
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
            .invoke(
                "NotifyServerDynamicSinks",
                (start, req_msg, client_site, _cordl_async),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        objectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objectUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChannelSink(
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
        > = __cordl_object.invoke("get_ChannelSink", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ClientDynamicProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection,
        > = __cordl_object.invoke("get_ClientDynamicProperties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Disposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EnvoySink(
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
        > = __cordl_object.invoke("get_EnvoySink", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasServerDynamicSinks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasServerDynamicSinks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsConnected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ObjectUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ChannelSink(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ChannelSink", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Disposed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ObjectUri(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ObjectUri", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Identity")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Runtime::Remoting::Identity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
