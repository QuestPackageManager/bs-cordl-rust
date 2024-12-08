#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicSegmentedControlController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _segmentedControl: *mut crate::HMUI::IconSegmentedControl,
    pub didSelectBeatmapCharacteristicEvent: *mut crate::System::Action_2<
        *mut BeatmapCharacteristicSegmentedControlController,
        *mut BeatmapCharacteristicSO,
    >,
    pub _selectedBeatmapCharacteristic: *mut BeatmapCharacteristicSO,
    pub _beatmapCharacteristics: *mut crate::System::Collections::Generic::List_1<
        *mut BeatmapCharacteristicSO,
    >,
}
#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapCharacteristicSegmentedControlController => ""
    ."BeatmapCharacteristicSegmentedControlController"
);
#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
impl std::ops::Deref for BeatmapCharacteristicSegmentedControlController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
impl std::ops::DerefMut for BeatmapCharacteristicSegmentedControlController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
impl BeatmapCharacteristicSegmentedControlController {
    #[cfg(feature = "BeatmapCharacteristicSegmentedControlController+__c")]
    pub type __c = crate::GlobalNamespace::BeatmapCharacteristicSegmentedControlController___c;
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
    pub fn HandleBeatmapCharacteristicSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: *mut crate::HMUI::SegmentedControl,
        cellIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapCharacteristicSegmentedControlDidSelectCell",
                (segmentedControl, cellIdx),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
        beatmapCharacteristics: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut BeatmapCharacteristicSO,
        >,
        selectedBeatmapCharacteristic: *mut BeatmapCharacteristicSO,
        notAllowedCharacteristics: *mut crate::System::Collections::Generic::HashSet_1<
            *mut BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (
                    beatmapCharacteristics,
                    selectedBeatmapCharacteristic,
                    notAllowedCharacteristics,
                ),
            )?;
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
    pub fn add_didSelectBeatmapCharacteristicEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut BeatmapCharacteristicSegmentedControlController,
            *mut BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectBeatmapCharacteristicEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedBeatmapCharacteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapCharacteristicSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapCharacteristicSO = __cordl_object
            .invoke("get_selectedBeatmapCharacteristic", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectBeatmapCharacteristicEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut BeatmapCharacteristicSegmentedControlController,
            *mut BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectBeatmapCharacteristicEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
impl quest_hook::libil2cpp::ObjectType
for BeatmapCharacteristicSegmentedControlController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
