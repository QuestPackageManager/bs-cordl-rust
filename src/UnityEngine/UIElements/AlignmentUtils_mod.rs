#[cfg(feature = "UnityEngine+UIElements+AlignmentUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct AlignmentUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+AlignmentUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::AlignmentUtils =>
    "UnityEngine.UIElements"."AlignmentUtils"
);
#[cfg(feature = "UnityEngine+UIElements+AlignmentUtils")]
impl std::ops::Deref for crate::UnityEngine::UIElements::AlignmentUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+AlignmentUtils")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::AlignmentUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+AlignmentUtils")]
impl crate::UnityEngine::UIElements::AlignmentUtils {
    pub fn CeilToPixelGrid(
        v: f32,
        pixelsPerPoint: f32,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CeilToPixelGrid", (v, pixelsPerPoint, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn RoundToPixelGrid(
        v: f32,
        pixelsPerPoint: f32,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RoundToPixelGrid", (v, pixelsPerPoint, offset))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+AlignmentUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::AlignmentUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
