#[cfg(feature = "RankModelHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct RankModelHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "RankModelHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for RankModelHelper => ""."RankModelHelper"
);
#[cfg(feature = "RankModelHelper")]
impl std::ops::Deref for RankModelHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RankModelHelper")]
impl std::ops::DerefMut for RankModelHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RankModelHelper")]
impl RankModelHelper {}
#[cfg(feature = "RankModelHelper")]
impl quest_hook::libil2cpp::ObjectType for RankModelHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
