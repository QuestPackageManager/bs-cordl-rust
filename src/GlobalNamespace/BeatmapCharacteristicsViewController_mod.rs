#[cfg(feature = "BeatmapCharacteristicsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapCharacteristicsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _beatmapCharacteristicsTableView: *mut crate::GlobalNamespace::BeatmapCharacteristicsTableView,
    pub didSelectBeatmapCharacteristicEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
    pub _beatmapCharacteristicCollection: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
    pub _selectedBeatmapCharacteristicNum: i32,
}
#[cfg(feature = "BeatmapCharacteristicsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapCharacteristicsViewController => ""
    ."BeatmapCharacteristicsViewController"
);
#[cfg(feature = "BeatmapCharacteristicsViewController")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapCharacteristicsViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicsViewController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapCharacteristicsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapCharacteristicsViewController")]
impl crate::GlobalNamespace::BeatmapCharacteristicsViewController {
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
    pub fn HandleBeatmapCharacteristicsTableViewDidSelecteCharacteristic(
        &mut self,
        beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapCharacteristicsTableViewDidSelecteCharacteristic",
                (beatmapCharacteristic),
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
    pub fn SetData(
        &mut self,
        beatmapCharacteristicCollection: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
        selectedCharacteristicNum: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (beatmapCharacteristicCollection, selectedCharacteristicNum),
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
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectBeatmapCharacteristicEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapCharacteristicCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection = __cordl_object
            .invoke("get_beatmapCharacteristicCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedBeatmapCharacteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapCharacteristicSO = __cordl_object
            .invoke("get_selectedBeatmapCharacteristic", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectBeatmapCharacteristicEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
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
#[cfg(feature = "BeatmapCharacteristicsViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapCharacteristicsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
