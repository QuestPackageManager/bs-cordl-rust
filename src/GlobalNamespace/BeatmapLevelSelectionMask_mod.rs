#[cfg(feature = "BeatmapLevelSelectionMask")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
    pub fn Deserialize(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        version: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapLevelSelectionMask,
    > {
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelSelectionMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deserialize", (reader, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_BeatmapLevelSelectionMask1(
        &mut self,
        other: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Gc0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        version: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer, version),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        l: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        r: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (l, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        l: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        r: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (l, r))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelSelectionMask")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelSelectionMask>>
for crate::GlobalNamespace::BeatmapLevelSelectionMask {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelSelectionMask> {
        todo!()
    }
}
#[cfg(feature = "BeatmapLevelSelectionMask")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelSelectionMask>>
for crate::GlobalNamespace::BeatmapLevelSelectionMask {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelSelectionMask,
    > {
        todo!()
    }
}
