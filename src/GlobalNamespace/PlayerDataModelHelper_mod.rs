#[cfg(feature = "PlayerDataModelHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerDataModelHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PlayerDataModelHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerDataModelHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerDataModelHelper";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "PlayerDataModelHelper")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerDataModelHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn ToPlayerAllOverallStatsData_PlayerAllOverallStatsData2(
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerDataModelHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerAllOverallStatsData,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData,
                >,
                1usize,
            >("ToPlayerAllOverallStatsData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PlayerDataModelHelper as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ToPlayerAllOverallStatsData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData,
        > = unsafe { method.invoke_unchecked((), (playerAllOverallStatsData))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToPlayerAllOverallStatsData_PlayerSaveDataV1_0_1_PlayerAllOverallStatsData1(
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerAllOverallStatsData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerDataModelHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerAllOverallStatsData,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerAllOverallStatsData,
                >,
                1usize,
            >("ToPlayerAllOverallStatsData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PlayerDataModelHelper as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ToPlayerAllOverallStatsData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData,
        > = unsafe { method.invoke_unchecked((), (playerAllOverallStatsData))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToPlayerAllOverallStatsData_PlayerSaveData_PlayerAllOverallStatsData0(
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerAllOverallStatsData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerDataModelHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerSaveData_PlayerAllOverallStatsData,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerAllOverallStatsData,
                >,
                1usize,
            >("ToPlayerAllOverallStatsData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PlayerDataModelHelper as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ToPlayerAllOverallStatsData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData,
        > = unsafe { method.invoke_unchecked((), (playerAllOverallStatsData))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerDataModelHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
                >,
                1usize,
            >("ToPlayerOverallStatsData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PlayerDataModelHelper as
                    quest_hook::libil2cpp::Type > ::class(), "ToPlayerOverallStatsData",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        > = unsafe { method.invoke_unchecked((), (playerOverallStatsData))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToPlayerOverallStats_PlayerSaveDataV1_0_1_PlayerOverallStatsData1(
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerDataModelHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerSaveDataV1_0_1_PlayerOverallStatsData,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
                >,
                1usize,
            >("ToPlayerOverallStats")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PlayerDataModelHelper as
                    quest_hook::libil2cpp::Type > ::class(), "ToPlayerOverallStats",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = unsafe { method.invoke_unchecked((), (playerAllOverallStatsData))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToPlayerOverallStats_PlayerSaveData_PlayerOverallStatsData0(
        playerAllOverallStatsData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerDataModelHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerSaveData_PlayerOverallStatsData,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
                >,
                1usize,
            >("ToPlayerOverallStats")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PlayerDataModelHelper as
                    quest_hook::libil2cpp::Type > ::class(), "ToPlayerOverallStats",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = unsafe { method.invoke_unchecked((), (playerAllOverallStatsData))? };
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
