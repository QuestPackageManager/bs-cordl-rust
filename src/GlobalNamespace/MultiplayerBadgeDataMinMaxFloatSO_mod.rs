#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgeDataMinMaxFloatSO {
    __cordl_parent: MultiplayerBadgeDataSO,
    pub _minMax: MultiplayerBadgeMinMax,
    pub _weightMultiplier: f32,
}
#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerBadgeDataMinMaxFloatSO => ""
    ."MultiplayerBadgeDataMinMaxFloatSO"
);
#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
impl std::ops::Deref for MultiplayerBadgeDataMinMaxFloatSO {
    type Target = MultiplayerBadgeDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
impl std::ops::DerefMut for MultiplayerBadgeDataMinMaxFloatSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
impl MultiplayerBadgeDataMinMaxFloatSO {
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
    pub fn CalculateMax(
        &mut self,
        resultsData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerPlayerResultsData,
        >,
        randomMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerBadgeAwardData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerBadgeAwardData = __cordl_object
            .invoke("CalculateMax", (resultsData, randomMultiplier))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateMin(
        &mut self,
        resultsData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerPlayerResultsData,
        >,
        randomMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerBadgeAwardData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerBadgeAwardData = __cordl_object
            .invoke("CalculateMin", (resultsData, randomMultiplier))?;
        Ok(__cordl_ret)
    }
    pub fn GetValue(
        &mut self,
        result: *mut MultiplayerPlayerResultsData,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetValue", (result))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerBadgeDataMinMaxFloatSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}