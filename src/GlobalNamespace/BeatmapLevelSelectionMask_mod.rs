#[cfg(feature = "BeatmapLevelSelectionMask")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapLevelSelectionMask {
    pub difficulties: crate::GlobalNamespace::BeatmapDifficultyMask,
    pub modifiers: crate::GlobalNamespace::GameplayModifierMask,
    pub songPacks: crate::GlobalNamespace::SongPackMask,
}
#[cfg(feature = "BeatmapLevelSelectionMask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelSelectionMask => ""
    ."BeatmapLevelSelectionMask"
);
#[cfg(feature = "BeatmapLevelSelectionMask")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapLevelSelectionMask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelSelectionMask")]
impl crate::GlobalNamespace::BeatmapLevelSelectionMask {
    pub fn Equals_BeatmapLevelSelectionMask1(
        &mut self,
        other: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
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
        difficulties: crate::GlobalNamespace::BeatmapDifficultyMask,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        songPacks: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (difficulties, modifiers, songPacks),
        )?;
        Ok(__cordl_ret)
    }
}
