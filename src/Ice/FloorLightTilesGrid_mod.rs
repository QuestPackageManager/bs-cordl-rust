#[cfg(feature = "Ice+FloorLightTilesGrid")]
#[repr(C)]
#[derive(Debug)]
pub struct FloorLightTilesGrid {
    __cordl_parent: AbstractPoolContainer,
    pub _floorLightTileMemoryPool: *mut crate::Ice::FloorLightTile_Pool,
    pub _floorLightTileMemoryPoolContainer: *mut MemoryPoolContainer_1<
        *mut crate::Ice::FloorLightTile,
    >,
    pub _grid: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::Ice::FloorLightTile>,
    >,
    pub _tileWidth: f32,
    pub _tileHeight: f32,
    pub _anchorPoint: crate::UnityEngine::Vector3,
    pub _ySize: i32,
}
#[cfg(feature = "Ice+FloorLightTilesGrid")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Ice::FloorLightTilesGrid => "Ice"
    ."FloorLightTilesGrid"
);
#[cfg(feature = "Ice+FloorLightTilesGrid")]
impl std::ops::Deref for crate::Ice::FloorLightTilesGrid {
    type Target = AbstractPoolContainer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+FloorLightTilesGrid")]
impl std::ops::DerefMut for crate::Ice::FloorLightTilesGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+FloorLightTilesGrid")]
impl crate::Ice::FloorLightTilesGrid {
    pub fn DespawnAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ySize", ())?;
        Ok(__cordl_ret)
    }
    pub fn DespawnTile(
        &mut self,
        floorLightTile: *mut crate::Ice::FloorLightTile,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnTile", (floorLightTile))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn HighlightTile(
        &mut self,
        x: i32,
        y: i32,
        fadeInDuration: f32,
        fadeOutDuration: f32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HighlightTile", (x, y, fadeInDuration, fadeOutDuration, color))?;
        Ok(__cordl_ret)
    }
    pub fn HandleFloorLightTileDidFinish(
        &mut self,
        floorLightTile: *mut crate::Ice::FloorLightTile,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFloorLightTileDidFinish", (floorLightTile))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        anchorPoint: crate::UnityEngine::Vector3,
        xSize: i32,
        ySize: i32,
        tileWidth: f32,
        tileHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (anchorPoint, xSize, ySize, tileWidth, tileHeight))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Ice+FloorLightTilesGrid")]
impl quest_hook::libil2cpp::ObjectType for crate::Ice::FloorLightTilesGrid {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
