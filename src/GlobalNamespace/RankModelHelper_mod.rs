#[cfg(feature = "RankModelHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct RankModelHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "RankModelHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RankModelHelper => ""
    ."RankModelHelper"
);
#[cfg(feature = "RankModelHelper")]
impl std::ops::Deref for crate::GlobalNamespace::RankModelHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RankModelHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::RankModelHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RankModelHelper")]
impl crate::GlobalNamespace::RankModelHelper {}
#[cfg(feature = "RankModelHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RankModelHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
