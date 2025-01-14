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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IEventHandler,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CaptureMouse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CaptureMouse", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handler))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasMouseCapture(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IEventHandler,
                >),
                bool,
                1usize,
            >("HasMouseCapture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HasMouseCapture", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (handler)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseMouse(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IEventHandler,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReleaseMouse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReleaseMouse", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handler))
        };
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
