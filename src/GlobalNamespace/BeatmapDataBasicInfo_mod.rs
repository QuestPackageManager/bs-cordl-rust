#[cfg(feature = "BeatmapDataBasicInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataBasicInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _numberOfLines_k__BackingField: i32,
    pub _cuttableNotesCount_k__BackingField: i32,
    pub _cuttableScoringObjectsCount_k__BackingField: i32,
    pub _obstaclesCount_k__BackingField: i32,
    pub _bombsCount_k__BackingField: i32,
}
#[cfg(feature = "BeatmapDataBasicInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataBasicInfo => ""
    ."BeatmapDataBasicInfo"
);
#[cfg(feature = "BeatmapDataBasicInfo")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataBasicInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataBasicInfo")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataBasicInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataBasicInfo")]
impl crate::GlobalNamespace::BeatmapDataBasicInfo {
    pub fn New(
        numberOfLines: i32,
        cuttableNotesCount: i32,
        cuttableScoringObjectsCount: i32,
        obstaclesCount: i32,
        bombsCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    numberOfLines,
                    cuttableNotesCount,
                    cuttableScoringObjectsCount,
                    obstaclesCount,
                    bombsCount,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        numberOfLines: i32,
        cuttableNotesCount: i32,
        cuttableScoringObjectsCount: i32,
        obstaclesCount: i32,
        bombsCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    numberOfLines,
                    cuttableNotesCount,
                    cuttableScoringObjectsCount,
                    obstaclesCount,
                    bombsCount,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bombsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_bombsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cuttableNotesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cuttableNotesCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cuttableScoringObjectsCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_cuttableScoringObjectsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_numberOfLines(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberOfLines", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_obstaclesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_obstaclesCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataBasicInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapDataBasicInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
