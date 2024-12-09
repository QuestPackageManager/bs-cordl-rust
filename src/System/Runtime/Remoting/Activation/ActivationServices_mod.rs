#[cfg(feature = "System+Runtime+Remoting+Activation+ActivationServices")]
#[repr(C)]
#[derive(Debug)]
pub struct ActivationServices {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::System::Runtime::Remoting::Activation::ActivationServices {}
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
