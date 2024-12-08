#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterSpawnEventValueParser {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BTSCharacterSpawnEventValueParser => ""
    ."BTSCharacterSpawnEventValueParser"
);
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
impl std::ops::Deref for crate::GlobalNamespace::BTSCharacterSpawnEventValueParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
impl std::ops::DerefMut for crate::GlobalNamespace::BTSCharacterSpawnEventValueParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
impl crate::GlobalNamespace::BTSCharacterSpawnEventValueParser {
    pub const kAlternativeMaterialMask: i32 = 65536i32;
    pub const kAlternativeMaterialOffset: i32 = 16i32;
    pub const kAnimationBitOffset: i32 = 8i32;
    pub const kAnimationMask: i32 = 65280i32;
    pub const kPrefabBitOffset: i32 = 0i32;
    pub const kPrefabMask: i32 = 255i32;
}
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BTSCharacterSpawnEventValueParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
