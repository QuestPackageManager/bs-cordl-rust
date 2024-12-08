#[cfg(feature = "UnityEngine+EnumDataUtility+CachedType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumDataUtility_CachedType {
    ExcludeObsolete = 0i32,
    IncludeAllObsolete = 2i32,
    IncludeObsoleteExceptErrors = 1i32,
}
#[cfg(feature = "UnityEngine+EnumDataUtility+CachedType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EnumDataUtility_CachedType =>
    "UnityEngine"."EnumDataUtility/CachedType"
);
#[cfg(feature = "UnityEngine+EnumDataUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumDataUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+EnumDataUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EnumDataUtility => "UnityEngine"
    ."EnumDataUtility"
);
#[cfg(feature = "UnityEngine+EnumDataUtility")]
impl std::ops::Deref for crate::UnityEngine::EnumDataUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EnumDataUtility")]
impl std::ops::DerefMut for crate::UnityEngine::EnumDataUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EnumDataUtility")]
impl crate::UnityEngine::EnumDataUtility {
    #[cfg(feature = "UnityEngine+EnumDataUtility+__c")]
    pub type __c = crate::UnityEngine::EnumDataUtility___c;
    #[cfg(feature = "UnityEngine+EnumDataUtility+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::UnityEngine::EnumDataUtility___c__DisplayClass2_0;
    #[cfg(feature = "UnityEngine+EnumDataUtility+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::UnityEngine::EnumDataUtility___c__DisplayClass8_0;
    #[cfg(feature = "UnityEngine+EnumDataUtility+CachedType")]
    pub type CachedType = crate::UnityEngine::EnumDataUtility_CachedType;
}
#[cfg(feature = "UnityEngine+EnumDataUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::EnumDataUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
