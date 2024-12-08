#[cfg(feature = "BeatmapCharacteristicSelectionViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicSelectionViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _beatmapCharacteristicSegmentedControl: *mut crate::HMUI::IconSegmentedControl,
    pub _beatmapCharacteristicCollection: *mut BeatmapCharacteristicCollection,
    pub didSelectBeatmapCharacteristicEvent: *mut crate::System::Action_2<
        *mut BeatmapCharacteristicSelectionViewController,
        *mut BeatmapCharacteristicSO,
    >,
    pub _selectedBeatmapCharacteristic: *mut BeatmapCharacteristicSO,
}
#[cfg(feature = "BeatmapCharacteristicSelectionViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapCharacteristicSelectionViewController => ""
    ."BeatmapCharacteristicSelectionViewController"
);
#[cfg(feature = "BeatmapCharacteristicSelectionViewController")]
impl std::ops::Deref for BeatmapCharacteristicSelectionViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicSelectionViewController")]
impl std::ops::DerefMut for BeatmapCharacteristicSelectionViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicSelectionViewController")]
impl BeatmapCharacteristicSelectionViewController {
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
    pub fn remove_didSelectBeatmapCharacteristicEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut BeatmapCharacteristicSelectionViewController,
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
    pub fn add_didSelectBeatmapCharacteristicEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut BeatmapCharacteristicSelectionViewController,
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
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
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
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapCharacteristicSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: *mut crate::HMUI::SegmentedControl,
        cellNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapCharacteristicSegmentedControlDidSelectCell",
                (segmentedControl, cellNumber),
            )?;
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
#[cfg(feature = "BeatmapCharacteristicSelectionViewController")]
impl quest_hook::libil2cpp::ObjectType for BeatmapCharacteristicSelectionViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
