#[cfg(feature = "System+Runtime+Remoting+SingleCallIdentity")]
#[repr(C)]
#[derive(Debug)]
pub struct SingleCallIdentity {
    __cordl_parent: crate::System::Runtime::Remoting::ServerIdentity,
}
#[cfg(feature = "System+Runtime+Remoting+SingleCallIdentity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::SingleCallIdentity =>
    "System.Runtime.Remoting"."SingleCallIdentity"
);
#[cfg(feature = "System+Runtime+Remoting+SingleCallIdentity")]
impl std::ops::Deref for crate::System::Runtime::Remoting::SingleCallIdentity {
    type Target = crate::System::Runtime::Remoting::ServerIdentity;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+SingleCallIdentity")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::SingleCallIdentity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+SingleCallIdentity")]
impl crate::System::Runtime::Remoting::SingleCallIdentity {
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
#[cfg(feature = "System+Runtime+Remoting+SingleCallIdentity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::SingleCallIdentity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
