#[cfg(feature = "System+Runtime+Remoting+ServerIdentity")]
#[repr(C)]
#[derive(Debug)]
pub struct ServerIdentity {
    __cordl_parent: crate::System::Runtime::Remoting::Identity,
    pub _objectType: *mut crate::System::Type,
    pub _serverObject: *mut crate::System::MarshalByRefObject,
    pub _serverSink: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    pub _context: *mut crate::System::Runtime::Remoting::Contexts::Context,
    pub _lease: *mut crate::System::Runtime::Remoting::Lifetime::Lease,
}
#[cfg(feature = "System+Runtime+Remoting+ServerIdentity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::ServerIdentity =>
    "System.Runtime.Remoting"."ServerIdentity"
);
#[cfg(feature = "System+Runtime+Remoting+ServerIdentity")]
impl std::ops::Deref for crate::System::Runtime::Remoting::ServerIdentity {
    type Target = crate::System::Runtime::Remoting::Identity;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ServerIdentity")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::ServerIdentity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+ServerIdentity")]
impl crate::System::Runtime::Remoting::ServerIdentity {
    pub fn AttachServerObject(
        &mut self,
        serverObject: *mut crate::System::MarshalByRefObject,
        context: *mut crate::System::Runtime::Remoting::Contexts::Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttachServerObject", (serverObject, context))?;
        Ok(__cordl_ret)
    }
    pub fn get_Context(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Contexts::Context,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Contexts::Context = __cordl_object
            .invoke("get_Context", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Context(
        &mut self,
        value: *mut crate::System::Runtime::Remoting::Contexts::Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Context", (value))?;
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
    pub fn DisposeServerObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisposeServerObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartTrackingLifetime(
        &mut self,
        lease: *mut crate::System::Runtime::Remoting::Lifetime::ILease,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartTrackingLifetime", (lease))?;
        Ok(__cordl_ret)
    }
    pub fn get_Lease(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Lifetime::Lease,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Lifetime::Lease = __cordl_object
            .invoke("get_Lease", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        objectUri: *mut crate::System::String,
        context: *mut crate::System::Runtime::Remoting::Contexts::Context,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objectUri, context, objectType))?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ObjectType", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateObjRef(
        &mut self,
        requestedType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Runtime::Remoting::ObjRef> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::ObjRef = __cordl_object
            .invoke("CreateObjRef", (requestedType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        objectUri: *mut crate::System::String,
        context: *mut crate::System::Runtime::Remoting::Contexts::Context,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectUri, context, objectType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+ServerIdentity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::ServerIdentity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
