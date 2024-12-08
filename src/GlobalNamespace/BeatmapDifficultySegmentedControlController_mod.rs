#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultySegmentedControlController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _difficultySegmentedControl: *mut crate::HMUI::TextSegmentedControl,
    pub didSelectDifficultyEvent: *mut crate::System::Action_2<
        *mut BeatmapDifficultySegmentedControlController,
        BeatmapDifficulty,
    >,
    pub _difficulties: *mut crate::System::Collections::Generic::List_1<
        BeatmapDifficulty,
    >,
    pub _selectedDifficulty: BeatmapDifficulty,
}
#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapDifficultySegmentedControlController => ""
    ."BeatmapDifficultySegmentedControlController"
);
#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
impl std::ops::Deref for BeatmapDifficultySegmentedControlController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
impl std::ops::DerefMut for BeatmapDifficultySegmentedControlController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
impl BeatmapDifficultySegmentedControlController {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedDifficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapDifficulty = __cordl_object
            .invoke("get_selectedDifficulty", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        difficultyBeatmaps: *mut crate::System::Collections::Generic::IEnumerable_1<
            BeatmapDifficulty,
        >,
        selectedDifficulty: BeatmapDifficulty,
        allowedBeatmapDifficultyMask: BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (difficultyBeatmaps, selectedDifficulty, allowedBeatmapDifficultyMask),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleDifficultySegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: *mut crate::HMUI::SegmentedControl,
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
        Ok(__cordl_ret)
    }
    pub fn add_didSelectDifficultyEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut BeatmapDifficultySegmentedControlController,
            BeatmapDifficulty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectDifficultyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectDifficultyEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut BeatmapDifficultySegmentedControlController,
            BeatmapDifficulty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectDifficultyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetClosestDifficultyIndex(
        &mut self,
        searchDifficulty: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetClosestDifficultyIndex", (searchDifficulty))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDifficultySegmentedControlController")]
impl quest_hook::libil2cpp::ObjectType for BeatmapDifficultySegmentedControlController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
