#[cfg(feature = "BeatmapLevelSelectionMask")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapLevelSelectionMask {
    pub difficulties: BeatmapDifficultyMask,
    pub modifiers: GameplayModifierMask,
    pub songPacks: SongPackMask,
}
#[cfg(feature = "BeatmapLevelSelectionMask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for BeatmapLevelSelectionMask => ""
    ."BeatmapLevelSelectionMask"
);
#[cfg(feature = "BeatmapLevelSelectionMask")]
unsafe impl quest_hook::libil2cpp::ThisArgument for BeatmapLevelSelectionMask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelSelectionMask")]
impl BeatmapLevelSelectionMask {
    pub fn Equals_BeatmapLevelSelectionMask1(
        &mut self,
        other: BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        version: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer, version),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        difficulties: BeatmapDifficultyMask,
        modifiers: GameplayModifierMask,
        songPacks: SongPackMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (difficulties, modifiers, songPacks),
        )?;
        Ok(__cordl_ret)
    }
}
