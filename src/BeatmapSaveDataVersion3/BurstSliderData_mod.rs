#[cfg(feature = "BeatmapSaveDataVersion3+BurstSliderData")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstSliderData {
    __cordl_parent: crate::BeatmapSaveDataVersion3::BaseSliderData,
    pub sc: i32,
    pub s: f32,
}
#[cfg(feature = "BeatmapSaveDataVersion3+BurstSliderData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion3::BurstSliderData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "BurstSliderData";
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_cuttableSlicesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cuttableSlicesCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sliceCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sliceCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_squishAmount(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_squishAmount", ())?;
        Ok(__cordl_ret.into())
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
