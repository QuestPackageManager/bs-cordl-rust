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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::BeatmapSaveDataCommon::NoteColorType,
                    f32,
                    i32,
                    i32,
                    crate::BeatmapSaveDataCommon::NoteCutDirection,
                    f32,
                    i32,
                    i32,
                    i32,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                10usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 10usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_cuttableSlicesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_cuttableSlicesCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_cuttableSlicesCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_sliceCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_sliceCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_sliceCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_squishAmount(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_squishAmount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_squishAmount", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
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
