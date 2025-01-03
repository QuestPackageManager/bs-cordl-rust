#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TerrainUtils::TerrainUtility =>
    "UnityEngine.TerrainUtils"."TerrainUtility"
);
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl std::ops::Deref for crate::UnityEngine::TerrainUtils::TerrainUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl std::ops::DerefMut for crate::UnityEngine::TerrainUtils::TerrainUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl crate::UnityEngine::TerrainUtils::TerrainUtility {
    pub fn AutoConnect() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AutoConnect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearConnectivity() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearConnectivity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CollectTerrains(
        onlyAutoConnectedTerrains: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                *mut crate::UnityEngine::TerrainUtils::TerrainMap,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                *mut crate::UnityEngine::TerrainUtils::TerrainMap,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CollectTerrains", (onlyAutoConnectedTerrains))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidTerrainsExist() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidTerrainsExist", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TerrainUtils::TerrainUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
