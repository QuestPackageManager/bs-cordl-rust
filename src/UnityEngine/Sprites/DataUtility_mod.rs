#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct DataUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Sprites::DataUtility =>
    "UnityEngine.Sprites"."DataUtility"
);
#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
impl std::ops::Deref for crate::UnityEngine::Sprites::DataUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Sprites::DataUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
impl crate::UnityEngine::Sprites::DataUtility {}
#[cfg(feature = "UnityEngine+Sprites+DataUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Sprites::DataUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
