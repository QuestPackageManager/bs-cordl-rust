#[cfg(feature = "MultiplayerBadgeDataFullComboSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgeDataFullComboSO {
    __cordl_parent: MultiplayerBadgeDataSO,
    pub _weight: f32,
}
#[cfg(feature = "MultiplayerBadgeDataFullComboSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerBadgeDataFullComboSO => ""
    ."MultiplayerBadgeDataFullComboSO"
);
#[cfg(feature = "MultiplayerBadgeDataFullComboSO")]
impl std::ops::Deref for MultiplayerBadgeDataFullComboSO {
    type Target = MultiplayerBadgeDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataFullComboSO")]
impl std::ops::DerefMut for MultiplayerBadgeDataFullComboSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataFullComboSO")]
impl MultiplayerBadgeDataFullComboSO {
    pub fn CalculateBadgeData(
        &mut self,
        resultsData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerPlayerResultsData,
        >,
        playerDataModel: *mut PlayerDataModel,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        randomMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerBadgeAwardData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerBadgeAwardData = __cordl_object
            .invoke(
                "CalculateBadgeData",
                (resultsData, playerDataModel, beatmapKey, randomMultiplier),
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerBadgeDataFullComboSO")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerBadgeDataFullComboSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
