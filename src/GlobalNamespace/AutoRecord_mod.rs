#[cfg(feature = "AutoRecord")]
#[repr(C)]
#[derive(Debug)]
pub struct AutoRecord {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AutoRecord")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AutoRecord => ""."AutoRecord"
);
#[cfg(feature = "AutoRecord")]
impl std::ops::Deref for crate::GlobalNamespace::AutoRecord {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AutoRecord")]
impl std::ops::DerefMut for crate::GlobalNamespace::AutoRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AutoRecord")]
impl crate::GlobalNamespace::AutoRecord {
    #[cfg(feature = "AutoRecord+Beatmap")]
    pub type Beatmap = crate::GlobalNamespace::AutoRecord_Beatmap;
    pub fn AddNoteHandFrames(
        note: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
        noteLineCount: i32,
        cutStart: f32,
        cutEnd: f32,
        trackOrientation: crate::UnityEngine::Quaternion,
        handFrames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Collections::Generic::List_1<
                    crate::BeatSaber::RecPlay::PoseFrame,
                >,
            >,
        >,
        lastCutDirections: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::NoteCutDirection>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddNoteHandFrames",
                (
                    note,
                    noteLineCount,
                    cutStart,
                    cutEnd,
                    trackOrientation,
                    handFrames,
                    lastCutDirections,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSliderHandFrames(
        slider: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
        noteLineCount: i32,
        cutStart: f32,
        cutEnd: f32,
        trackOrientation: crate::UnityEngine::Quaternion,
        handFrames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Collections::Generic::List_1<
                    crate::BeatSaber::RecPlay::PoseFrame,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddSliderHandFrames",
                (slider, noteLineCount, cutStart, cutEnd, trackOrientation, handFrames),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePlayerPoseFrames(
        beatmap: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::AutoRecord_Beatmap,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::RecPlay::PlayerPoseFrames> {
        let __cordl_ret: crate::BeatSaber::RecPlay::PlayerPoseFrames = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePlayerPoseFrames", (beatmap))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocateCutPosition(
        lineCount: i32,
        line: i32,
        layer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocateCutPosition", (lineCount, line, layer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _CreatePlayerPoseFrames_g__FixFrames_1_0(
        frames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::BeatSaber::RecPlay::PoseFrame>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("<CreatePlayerPoseFrames>g__FixFrames|1_0", (frames))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AutoRecord")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AutoRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AutoRecord+Beatmap")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AutoRecord_Beatmap {
    pub beatsPerMinute: f32,
    pub noteLineCount: i32,
    pub items: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BeatmapDataItem,
    >,
}
#[cfg(feature = "AutoRecord+Beatmap")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AutoRecord_Beatmap => ""
    ."AutoRecord/Beatmap"
);
#[cfg(feature = "AutoRecord+Beatmap")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::AutoRecord_Beatmap {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "AutoRecord+Beatmap")]
impl crate::GlobalNamespace::AutoRecord_Beatmap {}
