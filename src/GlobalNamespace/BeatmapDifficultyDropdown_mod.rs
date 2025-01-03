#[cfg(feature = "BeatmapDifficultyDropdown")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultyDropdown {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _simpleTextDropdown: *mut crate::HMUI::SimpleTextDropdown,
    pub didSelectCellWithIdxEvent: *mut crate::System::Action_1<i32>,
    pub _beatmapDifficultyData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::System::Tuple_2<
            crate::GlobalNamespace::BeatmapDifficultyMask,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _includeAllDifficulties_k__BackingField: bool,
}
#[cfg(feature = "BeatmapDifficultyDropdown")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDifficultyDropdown => ""
    ."BeatmapDifficultyDropdown"
);
#[cfg(feature = "BeatmapDifficultyDropdown")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDifficultyDropdown {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyDropdown")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDifficultyDropdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyDropdown")]
impl crate::GlobalNamespace::BeatmapDifficultyDropdown {
    pub fn GetIdxForBeatmapDifficultyMask(
        &mut self,
        beatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetIdxForBeatmapDifficultyMask", (beatmapDifficultyMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedBeatmapDifficultyMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficultyMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficultyMask = __cordl_object
            .invoke("GetSelectedBeatmapDifficultyMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSimpleTextDropdownDidSelectCellWithIdx(
        &mut self,
        dropdownWithTableView: quest_hook::libil2cpp::Gc<
            crate::HMUI::DropdownWithTableView,
        >,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSimpleTextDropdownDidSelectCellWithIdx",
                (dropdownWithTableView, idx),
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
    pub fn SelectCellWithBeatmapDifficultyMask(
        &mut self,
        beatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectCellWithBeatmapDifficultyMask", (beatmapDifficultyMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn add_didSelectCellWithIdxEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectCellWithIdxEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapDifficultyData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Tuple_2<
                    crate::GlobalNamespace::BeatmapDifficultyMask,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Tuple_2<
                    crate::GlobalNamespace::BeatmapDifficultyMask,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        > = __cordl_object.invoke("get_beatmapDifficultyData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_includeAllDifficulties(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_includeAllDifficulties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectCellWithIdxEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectCellWithIdxEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_includeAllDifficulties(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_includeAllDifficulties", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDifficultyDropdown")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDifficultyDropdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
