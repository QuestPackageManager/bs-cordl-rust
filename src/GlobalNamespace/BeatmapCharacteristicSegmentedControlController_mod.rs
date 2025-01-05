#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicSegmentedControlController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _segmentedControl: quest_hook::libil2cpp::Gc<crate::HMUI::IconSegmentedControl>,
    pub didSelectBeatmapCharacteristicEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BeatmapCharacteristicSegmentedControlController,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    >,
    pub _selectedBeatmapCharacteristic: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
    pub _currentlyAvailableBeatmapCharacteristics: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    >,
}
#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapCharacteristicSegmentedControlController => ""
    ."BeatmapCharacteristicSegmentedControlController"
);
#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapCharacteristicSegmentedControlController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapCharacteristicSegmentedControlController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
impl crate::GlobalNamespace::BeatmapCharacteristicSegmentedControlController {
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
    pub fn HandleBeatmapCharacteristicSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl>,
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
        beatmapCharacteristics: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
            >,
        >,
        selectedBeatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        notAllowedCharacteristics: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
            >,
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
    pub fn add_didSelectBeatmapCharacteristicEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSegmentedControlController,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectBeatmapCharacteristicEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedBeatmapCharacteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        > = __cordl_object.invoke("get_selectedBeatmapCharacteristic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectBeatmapCharacteristicEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSegmentedControlController,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectBeatmapCharacteristicEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapCharacteristicSegmentedControlController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCharacteristicSegmentedControlController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
