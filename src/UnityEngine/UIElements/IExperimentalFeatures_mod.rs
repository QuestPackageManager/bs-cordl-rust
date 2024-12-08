#[cfg(feature = "UnityEngine+UIElements+IExperimentalFeatures")]
#[repr(C)]
#[derive(Debug)]
pub struct IExperimentalFeatures {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IExperimentalFeatures")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IExperimentalFeatures
    => "UnityEngine.UIElements"."IExperimentalFeatures"
);
#[cfg(feature = "UnityEngine+UIElements+IExperimentalFeatures")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IExperimentalFeatures {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IExperimentalFeatures")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IExperimentalFeatures {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IExperimentalFeatures")]
impl crate::UnityEngine::UIElements::IExperimentalFeatures {
    pub fn get_animation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::Experimental::ITransitionAnimations,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Experimental::ITransitionAnimations = __cordl_object
            .invoke("get_animation", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IExperimentalFeatures")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IExperimentalFeatures {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
