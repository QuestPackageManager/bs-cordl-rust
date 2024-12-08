#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::Utils::TypeUtils =>
    "System.Dynamic.Utils"."TypeUtils"
);
#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
impl std::ops::Deref for crate::System::Dynamic::Utils::TypeUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
impl std::ops::DerefMut for crate::System::Dynamic::Utils::TypeUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
impl crate::System::Dynamic::Utils::TypeUtils {
    #[cfg(feature = "System+Dynamic+Utils+TypeUtils+__c")]
    pub type __c = crate::System::Dynamic::Utils::TypeUtils___c;
}
#[cfg(feature = "System+Dynamic+Utils+TypeUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Dynamic::Utils::TypeUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
