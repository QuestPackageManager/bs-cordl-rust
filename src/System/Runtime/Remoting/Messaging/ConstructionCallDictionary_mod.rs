#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionCallDictionary")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstructionCallDictionary {
    __cordl_parent: crate::System::Runtime::Remoting::Messaging::MessageDictionary,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionCallDictionary")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::ConstructionCallDictionary =>
    "System.Runtime.Remoting.Messaging"."ConstructionCallDictionary"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionCallDictionary")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::ConstructionCallDictionary {
    type Target = crate::System::Runtime::Remoting::Messaging::MessageDictionary;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionCallDictionary")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::ConstructionCallDictionary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionCallDictionary")]
impl crate::System::Runtime::Remoting::Messaging::ConstructionCallDictionary {
    pub fn GetMethodProperty(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("GetMethodProperty", (key))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        message: *mut crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message))?;
        Ok(__cordl_object)
    }
    pub fn SetMethodProperty(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppString,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMethodProperty", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        message: *mut crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ConstructionCallDictionary")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::ConstructionCallDictionary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
