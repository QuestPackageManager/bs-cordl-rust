#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::Utils::TypeExtensions =>
    "System.Dynamic.Utils"."TypeExtensions"
);
#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
impl std::ops::Deref for crate::System::Dynamic::Utils::TypeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
impl std::ops::DerefMut for crate::System::Dynamic::Utils::TypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
impl crate::System::Dynamic::Utils::TypeExtensions {}
#[cfg(feature = "System+Dynamic+Utils+TypeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::Utils::TypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
