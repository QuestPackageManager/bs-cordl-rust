#[cfg(feature = "UnityEngine+Tilemaps+TileData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TileData {
    pub m_Sprite: i32,
    pub m_Color: crate::UnityEngine::Color,
    pub m_Transform: crate::UnityEngine::Matrix4x4,
    pub m_GameObject: i32,
    pub m_Flags: crate::UnityEngine::Tilemaps::TileFlags,
    pub m_ColliderType: crate::UnityEngine::Tilemaps::Tile_ColliderType,
}
#[cfg(feature = "UnityEngine+Tilemaps+TileData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Tilemaps::TileData =>
    "UnityEngine.Tilemaps"."TileData"
);
#[cfg(feature = "UnityEngine+Tilemaps+TileData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Tilemaps::TileData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TileData")]
impl crate::UnityEngine::Tilemaps::TileData {
    pub fn CreateDefault() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Tilemaps::TileData,
    > {
        let __cordl_ret: crate::UnityEngine::Tilemaps::TileData = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colliderType(
        &mut self,
        value: crate::UnityEngine::Tilemaps::Tile_ColliderType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_colliderType",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_color",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flags(
        &mut self,
        value: crate::UnityEngine::Tilemaps::TileFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_flags",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gameObject(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_gameObject",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sprite(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_sprite",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_transform(
        &mut self,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_transform",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
