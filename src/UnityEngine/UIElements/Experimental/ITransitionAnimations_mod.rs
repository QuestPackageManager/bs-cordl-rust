#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
#[repr(C)]
#[derive(Debug)]
pub struct ITransitionAnimations {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Experimental::ITransitionAnimations =>
    "UnityEngine.UIElements.Experimental"."ITransitionAnimations"
);
#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
impl crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
    pub fn Start(
        &mut self,
        to: crate::UnityEngine::UIElements::Experimental::StyleValues,
        durationMs: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
            crate::UnityEngine::UIElements::Experimental::StyleValues,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
            crate::UnityEngine::UIElements::Experimental::StyleValues,
        > = __cordl_object.invoke("Start", (to, durationMs))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
