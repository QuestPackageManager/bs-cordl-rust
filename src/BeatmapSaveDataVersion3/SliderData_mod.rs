#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderData {
    __cordl_parent: crate::BeatmapSaveDataVersion3::BaseSliderData,
    pub mu: f32,
    pub tmu: f32,
    pub tc: crate::BeatmapSaveDataCommon::NoteCutDirection,
    pub m: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
}
#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatmapSaveDataVersion3::SliderData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "SliderData";
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
#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::SliderData {
    type Target = crate::BeatmapSaveDataVersion3::BaseSliderData;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::SliderData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
impl crate::BeatmapSaveDataVersion3::SliderData {
    pub fn New(
        colorType: crate::BeatmapSaveDataCommon::NoteColorType,
        headBeat: f32,
        headLine: i32,
        headLayer: i32,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        tailBeat: f32,
        tailLine: i32,
        tailLayer: i32,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        sliderMidAnchorMode: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
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
                    headControlPointLengthMultiplier,
                    headCutDirection,
                    tailBeat,
                    tailLine,
                    tailLayer,
                    tailControlPointLengthMultiplier,
                    tailCutDirection,
                    sliderMidAnchorMode,
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
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        tailBeat: f32,
        tailLine: i32,
        tailLayer: i32,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        sliderMidAnchorMode: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::BeatmapSaveDataCommon::NoteColorType,
                            f32,
                            i32,
                            i32,
                            f32,
                            crate::BeatmapSaveDataCommon::NoteCutDirection,
                            f32,
                            i32,
                            i32,
                            f32,
                            crate::BeatmapSaveDataCommon::NoteCutDirection,
                            crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        colorType,
                        headBeat,
                        headLine,
                        headLayer,
                        headControlPointLengthMultiplier,
                        headCutDirection,
                        tailBeat,
                        tailLine,
                        tailLayer,
                        tailControlPointLengthMultiplier,
                        tailCutDirection,
                        sliderMidAnchorMode,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_headControlPointLengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >("get_headControlPointLengthMultiplier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_headControlPointLengthMultiplier", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sliderMidAnchorMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
                        0usize,
                    >("get_sliderMidAnchorMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_sliderMidAnchorMode", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::SliderMidAnchorMode = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailControlPointLengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >("get_tailControlPointLengthMultiplier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_tailControlPointLengthMultiplier", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteCutDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataCommon::NoteCutDirection,
                        0usize,
                    >("get_tailCutDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_tailCutDirection", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteCutDirection = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataVersion3::SliderData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
