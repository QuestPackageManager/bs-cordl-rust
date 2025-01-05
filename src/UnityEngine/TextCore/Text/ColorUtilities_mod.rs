#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::ColorUtilities =>
    "UnityEngine.TextCore.Text"."ColorUtilities"
);
#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::ColorUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::ColorUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
impl crate::UnityEngine::TextCore::Text::ColorUtilities {
    pub fn CompareColors(
        a: crate::UnityEngine::Color32,
        b: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareColors", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyColors(
        c1: crate::UnityEngine::Color32,
        c2: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyColors", (c1, c2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::ColorUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
