#[cfg(feature = "UnityEngine+Tilemaps+TilemapRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct TilemapRenderer {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
}
#[cfg(feature = "UnityEngine+Tilemaps+TilemapRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Tilemaps::TilemapRenderer =>
    "UnityEngine.Tilemaps"."TilemapRenderer"
);
#[cfg(feature = "UnityEngine+Tilemaps+TilemapRenderer")]
impl std::ops::Deref for crate::UnityEngine::Tilemaps::TilemapRenderer {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TilemapRenderer")]
impl std::ops::DerefMut for crate::UnityEngine::Tilemaps::TilemapRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TilemapRenderer")]
impl crate::UnityEngine::Tilemaps::TilemapRenderer {
    pub fn OnSpriteAtlasRegistered(
        &mut self,
        atlas: quest_hook::libil2cpp::Gc<crate::UnityEngine::U2D::SpriteAtlas>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSpriteAtlasRegistered", (atlas))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterSpriteAtlasRegistered(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterSpriteAtlasRegistered", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterSpriteAtlasRegistered(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterSpriteAtlasRegistered", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TilemapRenderer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Tilemaps::TilemapRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
