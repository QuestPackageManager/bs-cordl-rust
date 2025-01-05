#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerCaptureHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PointerCaptureHelper =>
    "UnityEngine.UIElements"."PointerCaptureHelper"
);
#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerCaptureHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerCaptureHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
impl crate::UnityEngine::UIElements::PointerCaptureHelper {
    pub fn ActivateCompatibilityMouseEvents(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ActivateCompatibilityMouseEvents", (panel, pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn CapturePointer(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IEventHandler,
        >,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CapturePointer", (handler, pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCapturingElement(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IEventHandler,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCapturingElement", (panel, pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStateFor(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerDispatchState>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerDispatchState,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStateFor", (handler))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasPointerCapture(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IEventHandler,
        >,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasPointerCapture", (handler, pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn PreventCompatibilityMouseEvents(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreventCompatibilityMouseEvents", (panel, pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPointerCapture(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessPointerCapture", (panel, pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleasePointer_IEventHandler0(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IEventHandler,
        >,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleasePointer", (handler, pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleasePointer_IPanel1(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleasePointer", (panel, pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSendCompatibilityMouseEvents(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPointerEvent>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldSendCompatibilityMouseEvents", (panel, evt))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PointerCaptureHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
