#[cfg(feature = "System+Runtime+Remoting+Messaging+MCMDictionary")]
#[repr(C)]
#[derive(Debug)]
pub struct MCMDictionary {
    __cordl_parent: crate::System::Runtime::Remoting::Messaging::MessageDictionary,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MCMDictionary")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::MCMDictionary =>
    "System.Runtime.Remoting.Messaging"."MCMDictionary"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+MCMDictionary")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::MCMDictionary {
    type Target = crate::System::Runtime::Remoting::Messaging::MessageDictionary;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MCMDictionary")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Messaging::MCMDictionary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MCMDictionary")]
impl crate::System::Runtime::Remoting::Messaging::MCMDictionary {
    pub fn _ctor(
        &mut self,
        message: *mut crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        message: *mut crate::System::Runtime::Remoting::Messaging::IMethodMessage,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+MCMDictionary")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::MCMDictionary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
