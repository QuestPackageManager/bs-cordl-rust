#[cfg(feature = "UnityEngine+ICanvasRaycastFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct ICanvasRaycastFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ICanvasRaycastFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ICanvasRaycastFilter =>
    "UnityEngine"."ICanvasRaycastFilter"
);
#[cfg(feature = "UnityEngine+ICanvasRaycastFilter")]
impl std::ops::Deref for crate::UnityEngine::ICanvasRaycastFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ICanvasRaycastFilter")]
impl std::ops::DerefMut for crate::UnityEngine::ICanvasRaycastFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ICanvasRaycastFilter")]
impl crate::UnityEngine::ICanvasRaycastFilter {
    pub fn IsRaycastLocationValid(
        &mut self,
        sp: crate::UnityEngine::Vector2,
        eventCamera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsRaycastLocationValid", (sp, eventCamera))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+ICanvasRaycastFilter")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ICanvasRaycastFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
