#[cfg(feature = "Ice+FloorLightTilesGrid")]
#[repr(C)]
#[derive(Debug)]
pub struct FloorLightTilesGrid {
    __cordl_parent: crate::GlobalNamespace::AbstractPoolContainer,
    pub _floorLightTileMemoryPool: quest_hook::libil2cpp::Gc<
        crate::Ice::FloorLightTile_Pool,
    >,
    pub _floorLightTileMemoryPoolContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MemoryPoolContainer_1<
            quest_hook::libil2cpp::Gc<crate::Ice::FloorLightTile>,
        >,
    >,
    pub _grid: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::Ice::FloorLightTile>,
                >,
            >,
        >,
    >,
    pub _tileWidth: f32,
    pub _tileHeight: f32,
    pub _anchorPoint: crate::UnityEngine::Vector3,
    pub _ySize: i32,
}
#[cfg(feature = "Ice+FloorLightTilesGrid")]
unsafe impl quest_hook::libil2cpp::Type for crate::Ice::FloorLightTilesGrid {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Ice";
    const CLASS_NAME: &'static str = "FloorLightTilesGrid";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Ice+FloorLightTilesGrid")]
impl std::ops::Deref for crate::Ice::FloorLightTilesGrid {
    type Target = crate::GlobalNamespace::AbstractPoolContainer;
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
        Ok(__cordl_ret.into())
    }
    pub fn DespawnTile(
        &mut self,
        floorLightTile: quest_hook::libil2cpp::Gc<crate::Ice::FloorLightTile>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DespawnTile", (floorLightTile))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleFloorLightTileDidFinish(
        &mut self,
        floorLightTile: quest_hook::libil2cpp::Gc<crate::Ice::FloorLightTile>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFloorLightTileDidFinish", (floorLightTile))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ySize", ())?;
        Ok(__cordl_ret.into())
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
