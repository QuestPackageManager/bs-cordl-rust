#[cfg(feature = "UnityEngine+TerrainCollider")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainCollider {
    __cordl_parent: crate::UnityEngine::Collider,
}
#[cfg(feature = "UnityEngine+TerrainCollider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TerrainCollider => "UnityEngine"
    ."TerrainCollider"
);
#[cfg(feature = "UnityEngine+TerrainCollider")]
impl std::ops::Deref for crate::UnityEngine::TerrainCollider {
    type Target = crate::UnityEngine::Collider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainCollider")]
impl std::ops::DerefMut for crate::UnityEngine::TerrainCollider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainCollider")]
impl crate::UnityEngine::TerrainCollider {
    pub fn set_terrainData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_terrainData", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TerrainCollider")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TerrainCollider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
