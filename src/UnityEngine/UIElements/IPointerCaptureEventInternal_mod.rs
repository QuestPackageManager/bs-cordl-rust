#[cfg(feature = "UnityEngine+UIElements+IPointerCaptureEventInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct IPointerCaptureEventInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IPointerCaptureEventInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::IPointerCaptureEventInternal => "UnityEngine.UIElements"
    ."IPointerCaptureEventInternal"
);
#[cfg(feature = "UnityEngine+UIElements+IPointerCaptureEventInternal")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IPointerCaptureEventInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IPointerCaptureEventInternal")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::IPointerCaptureEventInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IPointerCaptureEventInternal")]
impl crate::UnityEngine::UIElements::IPointerCaptureEventInternal {
    pub fn get_pointerId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_pointerId", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IPointerCaptureEventInternal")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IPointerCaptureEventInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
