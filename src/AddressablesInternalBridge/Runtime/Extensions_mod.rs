#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::AddressablesInternalBridge::Runtime::Extensions
    => "AddressablesInternalBridge.Runtime"."Extensions"
);
#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
impl std::ops::Deref for crate::AddressablesInternalBridge::Runtime::Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
impl std::ops::DerefMut for crate::AddressablesInternalBridge::Runtime::Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
impl crate::AddressablesInternalBridge::Runtime::Extensions {
    #[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions+__c")]
    pub type __c = crate::AddressablesInternalBridge::Runtime::Extensions___c;
}
#[cfg(feature = "AddressablesInternalBridge+Runtime+Extensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::AddressablesInternalBridge::Runtime::Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
