#[cfg(feature = "UnityEngine+UIElements+Experimental+IValueAnimationUpdate")]
#[repr(C)]
#[derive(Debug)]
pub struct IValueAnimationUpdate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+IValueAnimationUpdate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Experimental::IValueAnimationUpdate =>
    "UnityEngine.UIElements.Experimental"."IValueAnimationUpdate"
);
#[cfg(feature = "UnityEngine+UIElements+Experimental+IValueAnimationUpdate")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+IValueAnimationUpdate")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+IValueAnimationUpdate")]
impl crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate {
    pub fn Tick(
        &mut self,
        currentTimeMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", (currentTimeMs))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+IValueAnimationUpdate")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
