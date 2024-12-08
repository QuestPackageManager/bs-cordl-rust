#[cfg(feature = "RankModel+Rank")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankModel_Rank {
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
#[cfg(feature = "RankModel")]
#[repr(C)]
#[derive(Debug)]
pub struct RankModel {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "RankModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for RankModel => ""."RankModel"
);
#[cfg(feature = "RankModel")]
impl std::ops::Deref for RankModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RankModel")]
impl std::ops::DerefMut for RankModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RankModel")]
impl RankModel {
    #[cfg(feature = "RankModel+Rank")]
    pub type Rank = crate::GlobalNamespace::RankModel_Rank;
}
#[cfg(feature = "RankModel")]
impl quest_hook::libil2cpp::ObjectType for RankModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
