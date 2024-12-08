#[cfg(feature = "System+Dynamic+Utils+ContractUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ContractUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Dynamic+Utils+ContractUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::Utils::ContractUtils =>
    "System.Dynamic.Utils"."ContractUtils"
);
#[cfg(feature = "System+Dynamic+Utils+ContractUtils")]
impl std::ops::Deref for crate::System::Dynamic::Utils::ContractUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+ContractUtils")]
impl std::ops::DerefMut for crate::System::Dynamic::Utils::ContractUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+ContractUtils")]
impl crate::System::Dynamic::Utils::ContractUtils {}
#[cfg(feature = "System+Dynamic+Utils+ContractUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Dynamic::Utils::ContractUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
