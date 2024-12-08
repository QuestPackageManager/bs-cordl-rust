#[cfg(feature = "BPMChangeBeatmapEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct BPMChangeBeatmapEventData {
    __cordl_parent: crate::GlobalNamespace::BeatmapEventData,
    pub bpm: f32,
}
#[cfg(feature = "BPMChangeBeatmapEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BPMChangeBeatmapEventData => ""
    ."BPMChangeBeatmapEventData"
);
#[cfg(feature = "BPMChangeBeatmapEventData")]
impl std::ops::Deref for crate::GlobalNamespace::BPMChangeBeatmapEventData {
    type Target = crate::GlobalNamespace::BeatmapEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BPMChangeBeatmapEventData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BPMChangeBeatmapEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BPMChangeBeatmapEventData")]
impl crate::GlobalNamespace::BPMChangeBeatmapEventData {
    pub fn GetCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::BeatmapDataItem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapDataItem = __cordl_object
            .invoke("GetCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::BeatmapEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapEventData = __cordl_object
            .invoke("GetDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(_cordl_time: f32, bpm: f32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_time, bpm))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        bpm: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, bpm))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BPMChangeBeatmapEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BPMChangeBeatmapEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
