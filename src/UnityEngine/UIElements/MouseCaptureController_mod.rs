#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseCaptureController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::MouseCaptureController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "MouseCaptureController";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MouseCaptureController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MouseCaptureController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
impl crate::UnityEngine::UIElements::MouseCaptureController {
    pub fn CaptureMouse(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CaptureMouse", (handler))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasMouseCapture(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasMouseCapture", (handler))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseMouse(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseMouse", (handler))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MouseCaptureController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
