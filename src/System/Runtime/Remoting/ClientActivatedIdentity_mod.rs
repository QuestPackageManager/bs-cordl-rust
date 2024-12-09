#[cfg(feature = "System+Runtime+Remoting+ClientActivatedIdentity")]
#[repr(C)]
#[derive(Debug)]
pub struct ClientActivatedIdentity {
    __cordl_parent: crate::System::Runtime::Remoting::ServerIdentity,
    pub _targetThis: *mut crate::System::MarshalByRefObject,
}
#[cfg(feature = "System+Runtime+Remoting+ClientActivatedIdentity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::ClientActivatedIdentity => "System.Runtime.Remoting"
    ."ClientActivatedIdentity"
);
#[cfg(feature = "System+Runtime+Remoting+ClientActivatedIdentity")]
impl std::ops::Deref for crate::System::Runtime::Remoting::ClientActivatedIdentity {
    type Target = crate::System::Runtime::Remoting::ServerIdentity;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ClientActivatedIdentity")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::ClientActivatedIdentity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ClientActivatedIdentity")]
impl crate::System::Runtime::Remoting::ClientActivatedIdentity {
    pub fn AsyncObjectProcessMessage(
        &mut self,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMessage,
        replySink: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IMessageCtrl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IMessageCtrl = __cordl_object
            .invoke("AsyncObjectProcessMessage", (msg, replySink))?;
        Ok(__cordl_ret)
    }
    pub fn GetServerObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::MarshalByRefObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::MarshalByRefObject = __cordl_object
            .invoke("GetServerObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        objectUri: *mut quest_hook::libil2cpp::Il2CppString,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectUri, objectType))?;
        Ok(__cordl_object)
    }
    pub fn OnLifetimeExpired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLifetimeExpired", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetClientProxy(
        &mut self,
        obj: *mut crate::System::MarshalByRefObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetClientProxy", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn SyncObjectProcessMessage(
        &mut self,
        msg: *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IMessage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IMessage = __cordl_object
            .invoke("SyncObjectProcessMessage", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        objectUri: *mut quest_hook::libil2cpp::Il2CppString,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objectUri, objectType))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+ClientActivatedIdentity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::ClientActivatedIdentity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
