#[cfg(feature = "HMUI+RoundedCornersDirectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct RoundedCornersDirectionExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+RoundedCornersDirectionExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::RoundedCornersDirectionExtensions =>
    "HMUI"."RoundedCornersDirectionExtensions"
);
#[cfg(feature = "HMUI+RoundedCornersDirectionExtensions")]
impl std::ops::Deref for crate::HMUI::RoundedCornersDirectionExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+RoundedCornersDirectionExtensions")]
impl std::ops::DerefMut for crate::HMUI::RoundedCornersDirectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+RoundedCornersDirectionExtensions")]
impl crate::HMUI::RoundedCornersDirectionExtensions {
    pub fn GetFlipAndSymmetry(
        direction: crate::HMUI::RoundedCornersDirection,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFlipAndSymmetry", (direction))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+RoundedCornersDirectionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::HMUI::RoundedCornersDirectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
