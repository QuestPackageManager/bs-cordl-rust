#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
#[repr(C)]
#[derive(Debug)]
pub struct LifetimeServices {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Lifetime::LifetimeServices =>
    "System.Runtime.Remoting.Lifetime"."LifetimeServices"
);
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Lifetime::LifetimeServices {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Lifetime::LifetimeServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
impl crate::System::Runtime::Remoting::Lifetime::LifetimeServices {}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LifetimeServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Lifetime::LifetimeServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
