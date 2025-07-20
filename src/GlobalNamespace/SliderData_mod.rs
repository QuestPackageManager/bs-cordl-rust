#[cfg(feature = "SliderData")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderData {
    __cordl_parent: crate::GlobalNamespace::BeatmapObjectData,
    pub _colorType_k__BackingField: crate::GlobalNamespace::ColorType,
    pub _sliderType_k__BackingField: crate::GlobalNamespace::SliderData_Type,
    pub _hasHeadNote_k__BackingField: bool,
    pub _headControlPointLengthMultiplier_k__BackingField: f32,
    pub _headLineIndex_k__BackingField: i32,
    pub _headLineLayer_k__BackingField: crate::GlobalNamespace::NoteLineLayer,
    pub _headBeforeJumpLineLayer_k__BackingField: crate::GlobalNamespace::NoteLineLayer,
    pub _headCutDirection_k__BackingField: crate::GlobalNamespace::NoteCutDirection,
    pub _headCutDirectionAngleOffset_k__BackingField: f32,
    pub _hasTailNote_k__BackingField: bool,
    pub _tailTime_k__BackingField: f32,
    pub _tailRotation_k__BackingField: i32,
    pub _tailLineIndex_k__BackingField: i32,
    pub _tailControlPointLengthMultiplier_k__BackingField: f32,
    pub _tailLineLayer_k__BackingField: crate::GlobalNamespace::NoteLineLayer,
    pub _tailBeforeJumpLineLayer_k__BackingField: crate::GlobalNamespace::NoteLineLayer,
    pub _tailCutDirection_k__BackingField: crate::GlobalNamespace::NoteCutDirection,
    pub _tailCutDirectionAngleOffset_k__BackingField: f32,
    pub _midAnchorMode_k__BackingField: crate::GlobalNamespace::SliderMidAnchorMode,
    pub _sliceCount_k__BackingField: i32,
    pub _squishAmount_k__BackingField: f32,
}
#[cfg(feature = "SliderData")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SliderData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
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
#[cfg(feature = "SliderData")]
impl std::ops::Deref for crate::GlobalNamespace::SliderData {
    type Target = crate::GlobalNamespace::BeatmapObjectData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderData")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderData")]
impl crate::GlobalNamespace::SliderData {
    #[cfg(feature = "SliderData+Type")]
    pub type Type = crate::GlobalNamespace::SliderData_Type;
    pub fn CreateBurstSliderData(
        colorType: crate::GlobalNamespace::ColorType,
        headTime: f32,
        headBeat: f32,
        headRotation: i32,
        headLineIndex: i32,
        headLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headCutDirection: crate::GlobalNamespace::NoteCutDirection,
        tailTime: f32,
        tailRotation: i32,
        tailLineIndex: i32,
        tailLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
        sliceCount: i32,
        squishAmount: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::GlobalNamespace::ColorType,
                    f32,
                    f32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteCutDirection,
                    f32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteLineLayer,
                    i32,
                    f32,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
                15usize,
            >("CreateBurstSliderData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "CreateBurstSliderData", 15usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData> = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        colorType,
                        headTime,
                        headBeat,
                        headRotation,
                        headLineIndex,
                        headLineLayer,
                        headBeforeJumpLineLayer,
                        headCutDirection,
                        tailTime,
                        tailRotation,
                        tailLineIndex,
                        tailLineLayer,
                        tailBeforeJumpLineLayer,
                        sliceCount,
                        squishAmount,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSliderData(
        colorType: crate::GlobalNamespace::ColorType,
        headTime: f32,
        headBeat: f32,
        headRotation: i32,
        headLineIndex: i32,
        headLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::GlobalNamespace::NoteCutDirection,
        tailTime: f32,
        tailRotation: i32,
        tailLineIndex: i32,
        tailLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: crate::GlobalNamespace::NoteCutDirection,
        midAnchorMode: crate::GlobalNamespace::SliderMidAnchorMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::GlobalNamespace::ColorType,
                    f32,
                    f32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteLineLayer,
                    f32,
                    crate::GlobalNamespace::NoteCutDirection,
                    f32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteLineLayer,
                    f32,
                    crate::GlobalNamespace::NoteCutDirection,
                    crate::GlobalNamespace::SliderMidAnchorMode,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
                17usize,
            >("CreateSliderData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "CreateSliderData", 17usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData> = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        colorType,
                        headTime,
                        headBeat,
                        headRotation,
                        headLineIndex,
                        headLineLayer,
                        headBeforeJumpLineLayer,
                        headControlPointLengthMultiplier,
                        headCutDirection,
                        tailTime,
                        tailRotation,
                        tailLineIndex,
                        tailLineLayer,
                        tailBeforeJumpLineLayer,
                        tailControlPointLengthMultiplier,
                        tailCutDirection,
                        midAnchorMode,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                0usize,
            >("GetCopy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "GetCopy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Mirror(
        &mut self,
        lineCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Mirror")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "Mirror", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (lineCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        sliderType: crate::GlobalNamespace::SliderData_Type,
        colorType: crate::GlobalNamespace::ColorType,
        hasHeadNote: bool,
        headTime: f32,
        headBeat: f32,
        headRotation: i32,
        headLineIndex: i32,
        headLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::GlobalNamespace::NoteCutDirection,
        headCutDirectionAngleOffset: f32,
        hasTailNote: bool,
        tailTime: f32,
        tailRotation: i32,
        tailLineIndex: i32,
        tailLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: crate::GlobalNamespace::NoteCutDirection,
        tailCutDirectionAngleOffset: f32,
        midAnchorMode: crate::GlobalNamespace::SliderMidAnchorMode,
        sliceCount: i32,
        squishAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    sliderType,
                    colorType,
                    hasHeadNote,
                    headTime,
                    headBeat,
                    headRotation,
                    headLineIndex,
                    headLineLayer,
                    headBeforeJumpLineLayer,
                    headControlPointLengthMultiplier,
                    headCutDirection,
                    headCutDirectionAngleOffset,
                    hasTailNote,
                    tailTime,
                    tailRotation,
                    tailLineIndex,
                    tailLineLayer,
                    tailBeforeJumpLineLayer,
                    tailControlPointLengthMultiplier,
                    tailCutDirection,
                    tailCutDirectionAngleOffset,
                    midAnchorMode,
                    sliceCount,
                    squishAmount,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetCutDirectionAngleOffset(
        &mut self,
        headCutDirectionAngleOffset: f32,
        tailCutDirectionAngleOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetCutDirectionAngleOffset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "SetCutDirectionAngleOffset", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (headCutDirectionAngleOffset, tailCutDirectionAngleOffset),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetHasHeadNote(
        &mut self,
        hasHeadNote: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("SetHasHeadNote")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "SetHasHeadNote", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (hasHeadNote))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetHasTailNote(
        &mut self,
        hasTailNote: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("SetHasTailNote")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "SetHasTailNote", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (hasTailNote))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetHeadBeforeJumpLineLayer(
        &mut self,
        lineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NoteLineLayer),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetHeadBeforeJumpLineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "SetHeadBeforeJumpLineLayer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (lineLayer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTailBeforeJumpLineLayer(
        &mut self,
        lineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NoteLineLayer),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetTailBeforeJumpLineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "SetTailBeforeJumpLineLayer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (lineLayer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubtypeIdentifier(
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::ColorType),
                i32,
                1usize,
            >("SubtypeIdentifier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "SubtypeIdentifier", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (colorType))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sliderType: crate::GlobalNamespace::SliderData_Type,
        colorType: crate::GlobalNamespace::ColorType,
        hasHeadNote: bool,
        headTime: f32,
        headBeat: f32,
        headRotation: i32,
        headLineIndex: i32,
        headLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::GlobalNamespace::NoteCutDirection,
        headCutDirectionAngleOffset: f32,
        hasTailNote: bool,
        tailTime: f32,
        tailRotation: i32,
        tailLineIndex: i32,
        tailLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailBeforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: crate::GlobalNamespace::NoteCutDirection,
        tailCutDirectionAngleOffset: f32,
        midAnchorMode: crate::GlobalNamespace::SliderMidAnchorMode,
        sliceCount: i32,
        squishAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::SliderData_Type,
                    crate::GlobalNamespace::ColorType,
                    bool,
                    f32,
                    f32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteLineLayer,
                    f32,
                    crate::GlobalNamespace::NoteCutDirection,
                    f32,
                    bool,
                    f32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteLineLayer,
                    f32,
                    crate::GlobalNamespace::NoteCutDirection,
                    f32,
                    crate::GlobalNamespace::SliderMidAnchorMode,
                    i32,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                24usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 24usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        sliderType,
                        colorType,
                        hasHeadNote,
                        headTime,
                        headBeat,
                        headRotation,
                        headLineIndex,
                        headLineLayer,
                        headBeforeJumpLineLayer,
                        headControlPointLengthMultiplier,
                        headCutDirection,
                        headCutDirectionAngleOffset,
                        hasTailNote,
                        tailTime,
                        tailRotation,
                        tailLineIndex,
                        tailLineLayer,
                        tailBeforeJumpLineLayer,
                        tailControlPointLengthMultiplier,
                        tailCutDirection,
                        tailCutDirectionAngleOffset,
                        midAnchorMode,
                        sliceCount,
                        squishAmount,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_colorType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ColorType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::ColorType,
                0usize,
            >("get_colorType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_colorType", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::ColorType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasHeadNote(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_hasHeadNote")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_hasHeadNote", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasTailNote(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_hasTailNote")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_hasTailNote", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_headBeforeJumpLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::NoteLineLayer,
                0usize,
            >("get_headBeforeJumpLineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_headBeforeJumpLineLayer", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_headControlPointLengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_headControlPointLengthMultiplier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_headControlPointLengthMultiplier", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_headCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::NoteCutDirection,
                0usize,
            >("get_headCutDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_headCutDirection", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_headCutDirectionAngleOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_headCutDirectionAngleOffset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_headCutDirectionAngleOffset", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_headLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_headLineIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_headLineIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_headLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::NoteLineLayer,
                0usize,
            >("get_headLineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_headLineLayer", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_midAnchorMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SliderMidAnchorMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::SliderMidAnchorMode,
                0usize,
            >("get_midAnchorMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_midAnchorMode", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::SliderMidAnchorMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_sliceCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_sliceCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_sliceCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sliderType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SliderData_Type> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::SliderData_Type,
                0usize,
            >("get_sliderType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_sliderType", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::SliderData_Type = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_squishAmount(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_squishAmount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_squishAmount", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_subtypeGroupIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_subtypeGroupIdentifier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_subtypeGroupIdentifier", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailBeforeJumpLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::NoteLineLayer,
                0usize,
            >("get_tailBeforeJumpLineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_tailBeforeJumpLineLayer", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailControlPointLengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_tailControlPointLengthMultiplier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_tailControlPointLengthMultiplier", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::NoteCutDirection,
                0usize,
            >("get_tailCutDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_tailCutDirection", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailCutDirectionAngleOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_tailCutDirectionAngleOffset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_tailCutDirectionAngleOffset", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_tailLineIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_tailLineIndex", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::NoteLineLayer,
                0usize,
            >("get_tailLineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_tailLineLayer", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailRotation(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_tailRotation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_tailRotation", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_tailTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_tailTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "get_tailTime", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_colorType(
        &mut self,
        value: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::ColorType),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_colorType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_colorType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_hasHeadNote(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_hasHeadNote")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_hasHeadNote", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_hasTailNote(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_hasTailNote")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_hasTailNote", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_headBeforeJumpLineLayer(
        &mut self,
        value: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NoteLineLayer),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_headBeforeJumpLineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_headBeforeJumpLineLayer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_headControlPointLengthMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_headControlPointLengthMultiplier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_headControlPointLengthMultiplier", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_headCutDirection(
        &mut self,
        value: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NoteCutDirection),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_headCutDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_headCutDirection", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_headCutDirectionAngleOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_headCutDirectionAngleOffset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_headCutDirectionAngleOffset", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_headLineIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_headLineIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_headLineIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_headLineLayer(
        &mut self,
        value: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NoteLineLayer),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_headLineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_headLineLayer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_midAnchorMode(
        &mut self,
        value: crate::GlobalNamespace::SliderMidAnchorMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::SliderMidAnchorMode),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_midAnchorMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_midAnchorMode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_sliceCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_sliceCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_sliceCount", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_sliderType(
        &mut self,
        value: crate::GlobalNamespace::SliderData_Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::SliderData_Type),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_sliderType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_sliderType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_squishAmount(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_squishAmount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_squishAmount", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_tailBeforeJumpLineLayer(
        &mut self,
        value: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NoteLineLayer),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_tailBeforeJumpLineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_tailBeforeJumpLineLayer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_tailControlPointLengthMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_tailControlPointLengthMultiplier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_tailControlPointLengthMultiplier", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_tailCutDirection(
        &mut self,
        value: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NoteCutDirection),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_tailCutDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_tailCutDirection", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_tailCutDirectionAngleOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_tailCutDirectionAngleOffset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_tailCutDirectionAngleOffset", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_tailLineIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_tailLineIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_tailLineIndex", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_tailLineLayer(
        &mut self,
        value: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::NoteLineLayer),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_tailLineLayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_tailLineLayer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_tailRotation(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_tailRotation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_tailRotation", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_tailTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_tailTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SliderData as quest_hook::libil2cpp::Type >
                    ::class(), "set_tailTime", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SliderData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SliderData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SliderData+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderData_Type {
    #[default]
    Burst = 1i32,
    Normal = 0i32,
}
#[cfg(feature = "SliderData+Type")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SliderData_Type {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SliderData/Type";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "SliderData+Type")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::SliderData_Type {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "SliderData+Type")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::SliderData_Type {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "SliderData+Type")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::SliderData_Type {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "SliderData+Type")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::SliderData_Type {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
