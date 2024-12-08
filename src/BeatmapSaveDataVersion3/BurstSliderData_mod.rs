#[cfg(feature = "BeatmapSaveDataVersion3+BurstSliderData")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstSliderData {
    __cordl_parent: crate::BeatmapSaveDataVersion3::BaseSliderData,
    pub sc: i32,
    pub s: f32,
}
#[cfg(feature = "BeatmapSaveDataVersion3+BurstSliderData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::BurstSliderData =>
    "BeatmapSaveDataVersion3"."BurstSliderData"
);
#[cfg(feature = "BeatmapSaveDataVersion3+BurstSliderData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::BurstSliderData {
    type Target = crate::BeatmapSaveDataVersion3::BaseSliderData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BurstSliderData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::BurstSliderData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BurstSliderData")]
impl crate::BeatmapSaveDataVersion3::BurstSliderData {
    pub fn New(
        colorType: crate::BeatmapSaveDataCommon::NoteColorType,
        headBeat: f32,
        headLine: i32,
        headLayer: i32,
        headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        tailBeat: f32,
        tailLine: i32,
        tailLayer: i32,
        sliceCount: i32,
        squishAmount: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    colorType,
                    headBeat,
                    headLine,
                    headLayer,
                    headCutDirection,
                    tailBeat,
                    tailLine,
                    tailLayer,
                    sliceCount,
                    squishAmount,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        colorType: crate::BeatmapSaveDataCommon::NoteColorType,
        headBeat: f32,
        headLine: i32,
        headLayer: i32,
        headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        tailBeat: f32,
        tailLine: i32,
        tailLayer: i32,
        sliceCount: i32,
        squishAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    colorType,
                    headBeat,
                    headLine,
                    headLayer,
                    headCutDirection,
                    tailBeat,
                    tailLine,
                    tailLayer,
                    sliceCount,
                    squishAmount,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_sliceCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sliceCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_squishAmount(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_squishAmount", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BurstSliderData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::BurstSliderData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
