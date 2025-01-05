#[cfg(feature = "PlayerDataModelHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerDataModelHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "PlayerDataModelHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerDataModelHelper => ""
    ."PlayerDataModelHelper"
);
#[cfg(feature = "PlayerDataModelHelper")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerDataModelHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerDataModelHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerDataModelHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerDataModelHelper")]
impl crate::GlobalNamespace::PlayerDataModelHelper {
    pub fn ToPlayerAllOverallStatsData_Gc0(
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerAllOverallStatsData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToPlayerAllOverallStatsData", (playerAllOverallStatsData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToPlayerAllOverallStatsData_Gc1(
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerAllOverallStatsData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToPlayerAllOverallStatsData", (playerAllOverallStatsData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToPlayerAllOverallStatsData_Gc2(
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToPlayerAllOverallStatsData", (playerAllOverallStatsData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToPlayerOverallStatsData(
        playerOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToPlayerOverallStatsData", (playerOverallStatsData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToPlayerOverallStats_Gc0(
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToPlayerOverallStats", (playerAllOverallStatsData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToPlayerOverallStats_Gc1(
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToPlayerOverallStats", (playerAllOverallStatsData))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerDataModelHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerDataModelHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
