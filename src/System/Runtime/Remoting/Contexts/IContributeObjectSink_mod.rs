#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeObjectSink")]
#[repr(C)]
#[derive(Debug)]
pub struct IContributeObjectSink {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeObjectSink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Contexts::IContributeObjectSink =>
    "System.Runtime.Remoting.Contexts"."IContributeObjectSink"
);
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeObjectSink")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Contexts::IContributeObjectSink {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeObjectSink")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Contexts::IContributeObjectSink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeObjectSink")]
impl crate::System::Runtime::Remoting::Contexts::IContributeObjectSink {
    pub fn GetObjectSink(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::System::MarshalByRefObject>,
        nextSink: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
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
        > = __cordl_object.invoke("GetObjectSink", (obj, nextSink))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeObjectSink")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Contexts::IContributeObjectSink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
