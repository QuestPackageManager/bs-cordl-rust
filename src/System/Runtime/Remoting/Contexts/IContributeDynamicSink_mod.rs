#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeDynamicSink")]
#[repr(C)]
#[derive(Debug)]
pub struct IContributeDynamicSink {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeDynamicSink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Contexts::IContributeDynamicSink =>
    "System.Runtime.Remoting.Contexts"."IContributeDynamicSink"
);
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeDynamicSink")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Contexts::IContributeDynamicSink {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeDynamicSink")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Contexts::IContributeDynamicSink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeDynamicSink")]
impl crate::System::Runtime::Remoting::Contexts::IContributeDynamicSink {
    pub fn GetDynamicSink(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::IDynamicMessageSink,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::IDynamicMessageSink,
        > = __cordl_object.invoke("GetDynamicSink", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeDynamicSink")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Contexts::IContributeDynamicSink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
