#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterSpawnEventValueParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BTSCharacterSpawnEventValueParser => ""
    ."BTSCharacterSpawnEventValueParser"
);
#[cfg(feature = "BTSCharacterSpawnEventValueParser")]
impl std::ops::Deref for crate::GlobalNamespace::BTSCharacterSpawnEventValueParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetAnimationId(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAnimationId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIsAlternativeMaterial(value: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIsAlternativeMaterial", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrefabId(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrefabId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn MergeValuesIntoOneInt(
        prefabId: i32,
        animationId: i32,
        isAlternativeMaterial: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MergeValuesIntoOneInt",
                (prefabId, animationId, isAlternativeMaterial),
            )?;
        Ok(__cordl_ret.into())
    }
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
