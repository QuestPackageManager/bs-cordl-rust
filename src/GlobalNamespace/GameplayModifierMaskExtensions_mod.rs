#[cfg(feature = "GameplayModifierMaskExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifierMaskExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifierMaskExtensions
    => ""."GameplayModifierMaskExtensions"
);
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayModifierMaskExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayModifierMaskExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl crate::GlobalNamespace::GameplayModifierMaskExtensions {}
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifierMaskExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
