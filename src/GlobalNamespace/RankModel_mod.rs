#[cfg(feature = "RankModel")]
#[repr(C)]
#[derive(Debug)]
pub struct RankModel {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "RankModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RankModel => ""."RankModel"
);
#[cfg(feature = "RankModel")]
impl std::ops::Deref for crate::GlobalNamespace::RankModel {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RankModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::RankModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RankModel")]
impl crate::GlobalNamespace::RankModel {
    #[cfg(feature = "RankModel+Rank")]
    pub type Rank = crate::GlobalNamespace::RankModel_Rank;
    pub fn GetRankForScore(
        multipliedScore: i32,
        modifiedScore: i32,
        maxMultipliedScore: i32,
        maxModifiedScore: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::RankModel_Rank> {
        let __cordl_ret: crate::GlobalNamespace::RankModel_Rank = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetRankForScore",
                (multipliedScore, modifiedScore, maxMultipliedScore, maxModifiedScore),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRankName(
        rank: crate::GlobalNamespace::RankModel_Rank,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRankName", (rank))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RankModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RankModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RankModel+Rank")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RankModel_Rank {
    #[default]
    A = 4i32,
    B = 3i32,
    C = 2i32,
    D = 1i32,
    E = 0i32,
    S = 5i32,
    SS = 6i32,
    SSS = 7i32,
}
#[cfg(feature = "RankModel+Rank")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RankModel_Rank => ""
    ."RankModel/Rank"
);
