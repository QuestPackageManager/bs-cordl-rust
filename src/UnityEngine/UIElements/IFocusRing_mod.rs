#[cfg(feature = "UnityEngine+UIElements+IFocusRing")]
#[repr(C)]
#[derive(Debug)]
pub struct IFocusRing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IFocusRing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IFocusRing =>
    "UnityEngine.UIElements"."IFocusRing"
);
#[cfg(feature = "UnityEngine+UIElements+IFocusRing")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IFocusRing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IFocusRing")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IFocusRing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IFocusRing")]
impl crate::UnityEngine::UIElements::IFocusRing {
    pub fn GetNextFocusable(
        &mut self,
        currentFocusable: *mut crate::UnityEngine::UIElements::Focusable,
        direction: *mut crate::UnityEngine::UIElements::FocusChangeDirection,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::Focusable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Focusable = __cordl_object
            .invoke("GetNextFocusable", (currentFocusable, direction))?;
        Ok(__cordl_ret)
    }
    pub fn GetFocusChangeDirection(
        &mut self,
        currentFocusable: *mut crate::UnityEngine::UIElements::Focusable,
        e: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::FocusChangeDirection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::FocusChangeDirection = __cordl_object
            .invoke("GetFocusChangeDirection", (currentFocusable, e))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IFocusRing")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::IFocusRing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
