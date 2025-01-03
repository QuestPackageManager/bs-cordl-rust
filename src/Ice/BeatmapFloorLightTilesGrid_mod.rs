#[cfg(feature = "Ice+BeatmapFloorLightTilesGrid")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapFloorLightTilesGrid {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _numberOfRows: i32,
    pub _tileWidth: f32,
    pub _tileHeight: f32,
    pub _floorLightTilesGrid: quest_hook::libil2cpp::Gc<crate::Ice::FloorLightTilesGrid>,
    pub _beatmapObjectSpawnController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectSpawnController,
    >,
}
#[cfg(feature = "Ice+BeatmapFloorLightTilesGrid")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Ice::BeatmapFloorLightTilesGrid => "Ice"
    ."BeatmapFloorLightTilesGrid"
);
#[cfg(feature = "Ice+BeatmapFloorLightTilesGrid")]
impl std::ops::Deref for crate::Ice::BeatmapFloorLightTilesGrid {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+BeatmapFloorLightTilesGrid")]
impl std::ops::DerefMut for crate::Ice::BeatmapFloorLightTilesGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Ice+BeatmapFloorLightTilesGrid")]
impl crate::Ice::BeatmapFloorLightTilesGrid {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
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
}
#[cfg(feature = "Ice+BeatmapFloorLightTilesGrid")]
impl quest_hook::libil2cpp::ObjectType for crate::Ice::BeatmapFloorLightTilesGrid {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
