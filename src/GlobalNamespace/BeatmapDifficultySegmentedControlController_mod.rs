#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultySegmentedControlController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _difficultySegmentedControl: quest_hook::libil2cpp::Gc<
        crate::HMUI::TextSegmentedControl,
    >,
    pub didSelectDifficultyEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BeatmapDifficultySegmentedControlController,
            >,
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
    >,
    pub _difficulties: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
    >,
    pub _selectedDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
}
#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapDifficultySegmentedControlController => ""
    ."BeatmapDifficultySegmentedControlController"
);
#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapDifficultySegmentedControlController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapDifficultySegmentedControlController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
impl crate::GlobalNamespace::BeatmapDifficultySegmentedControlController {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClosestDifficultyIndex(
        &mut self,
        searchDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetClosestDifficultyIndex", (searchDifficulty))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDifficultySegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl>,
        cellIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleDifficultySegmentedControlDidSelectCell",
                (segmentedControl, cellIdx),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        difficultyBeatmaps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
        >,
        selectedDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        allowedBeatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (difficultyBeatmaps, selectedDifficulty, allowedBeatmapDifficultyMask),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didSelectDifficultyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapDifficultySegmentedControlController,
                >,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectDifficultyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedDifficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficulty = __cordl_object
            .invoke("get_selectedDifficulty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectDifficultyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapDifficultySegmentedControlController,
                >,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectDifficultyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDifficultySegmentedControlController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
