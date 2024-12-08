#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct FontEngine {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::LowLevel::FontEngine =>
    "UnityEngine.TextCore.LowLevel"."FontEngine"
);
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl std::ops::Deref for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl crate::UnityEngine::TextCore::LowLevel::FontEngine {}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
