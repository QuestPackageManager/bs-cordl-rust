#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderData {
    __cordl_parent: crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem,
    pub _colorType: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
    pub _headTime: f32,
    pub _headLineIndex: i32,
    pub _headLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
    pub _headControlPointLengthMultiplier: f32,
    pub _headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
    pub _tailTime: f32,
    pub _tailLineIndex: i32,
    pub _tailLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
    pub _tailControlPointLengthMultiplier: f32,
    pub _tailCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
    pub _sliderMidAnchorMode: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion2_6_0AndEarlier";
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
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData {
    type Target = crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
impl crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData {
    pub fn New(
        colorType: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
        headTime: f32,
        headLineIndex: i32,
        headLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        tailTime: f32,
        tailLineIndex: i32,
        tailLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
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
                    headTime,
                    headLineIndex,
                    headLineLayer,
                    headControlPointLengthMultiplier,
                    headCutDirection,
                    tailTime,
                    tailLineIndex,
                    tailLineLayer,
                    tailControlPointLengthMultiplier,
                    tailCutDirection,
                    sliderMidAnchorMode,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        colorType: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
        headTime: f32,
        headLineIndex: i32,
        headLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        tailTime: f32,
        tailLineIndex: i32,
        tailLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        sliderMidAnchorMode: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
                            f32,
                            i32,
                            crate::BeatmapSaveDataCommon::NoteLineLayer,
                            f32,
                            crate::BeatmapSaveDataCommon::NoteCutDirection,
                            f32,
                            i32,
                            crate::BeatmapSaveDataCommon::NoteLineLayer,
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
                            Self::class(), ".ctor", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        colorType,
                        headTime,
                        headLineIndex,
                        headLineLayer,
                        headControlPointLengthMultiplier,
                        headCutDirection,
                        tailTime,
                        tailLineIndex,
                        tailLineLayer,
                        tailControlPointLengthMultiplier,
                        tailCutDirection,
                        sliderMidAnchorMode,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_colorType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
                        0usize,
                    >("get_colorType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_colorType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_headControlPointLengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >("get_headControlPointLengthMultiplier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_headControlPointLengthMultiplier", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_headCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteCutDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataCommon::NoteCutDirection,
                        0usize,
                    >("get_headCutDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_headCutDirection", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteCutDirection = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_headLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("get_headLineIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_headLineIndex", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_headLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteLineLayer> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataCommon::NoteLineLayer,
                        0usize,
                    >("get_headLineLayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_headLineLayer", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteLineLayer = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_sliderMidAnchorMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
                        0usize,
                    >("get_sliderMidAnchorMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_sliderMidAnchorMode", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::SliderMidAnchorMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailControlPointLengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >("get_tailControlPointLengthMultiplier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_tailControlPointLengthMultiplier", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteCutDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataCommon::NoteCutDirection,
                        0usize,
                    >("get_tailCutDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_tailCutDirection", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteCutDirection = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("get_tailLineIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_tailLineIndex", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteLineLayer> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::BeatmapSaveDataCommon::NoteLineLayer,
                        0usize,
                    >("get_tailLineLayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_tailLineLayer", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteLineLayer = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), f32, 0usize>("get_tailTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_tailTime", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), f32, 0usize>("get_time")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_time", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
