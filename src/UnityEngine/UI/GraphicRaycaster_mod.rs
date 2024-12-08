#[cfg(feature = "UnityEngine+UI+GraphicRaycaster+BlockingObjects")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicRaycaster_BlockingObjects {
    All = 3i32,
    None = 0i32,
    ThreeD = 2i32,
    TwoD = 1i32,
}
#[cfg(feature = "UnityEngine+UI+GraphicRaycaster+BlockingObjects")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UI::GraphicRaycaster_BlockingObjects => "UnityEngine.UI"
    ."GraphicRaycaster/BlockingObjects"
);
#[cfg(feature = "UnityEngine+UI+GraphicRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicRaycaster {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseRaycaster,
    pub m_IgnoreReversedGraphics: bool,
    pub m_BlockingObjects: crate::UnityEngine::UI::GraphicRaycaster_BlockingObjects,
    pub m_BlockingMask: crate::UnityEngine::LayerMask,
    pub m_Canvas: *mut crate::UnityEngine::Canvas,
    pub m_RaycastResults: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UI::Graphic,
    >,
}
#[cfg(feature = "UnityEngine+UI+GraphicRaycaster")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::GraphicRaycaster =>
    "UnityEngine.UI"."GraphicRaycaster"
);
#[cfg(feature = "UnityEngine+UI+GraphicRaycaster")]
impl std::ops::Deref for crate::UnityEngine::UI::GraphicRaycaster {
    type Target = crate::UnityEngine::EventSystems::BaseRaycaster;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+GraphicRaycaster")]
impl std::ops::DerefMut for crate::UnityEngine::UI::GraphicRaycaster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+GraphicRaycaster")]
impl crate::UnityEngine::UI::GraphicRaycaster {
    pub const kNoEventMaskSet: i32 = -1i32;
    #[cfg(feature = "UnityEngine+UI+GraphicRaycaster+BlockingObjects")]
    pub type BlockingObjects = crate::UnityEngine::UI::GraphicRaycaster_BlockingObjects;
    #[cfg(feature = "UnityEngine+UI+GraphicRaycaster+__c")]
    pub type __c = crate::UnityEngine::UI::GraphicRaycaster___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Raycast(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
        resultAppendList: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::EventSystems::RaycastResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Raycast", (eventData, resultAppendList))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_blockingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("get_blockingMask", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_blockingObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::GraphicRaycaster_BlockingObjects,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::GraphicRaycaster_BlockingObjects = __cordl_object
            .invoke("get_blockingObjects", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_canvas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Canvas> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Canvas = __cordl_object
            .invoke("get_canvas", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_eventCamera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Camera> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Camera = __cordl_object
            .invoke("get_eventCamera", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ignoreReversedGraphics(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ignoreReversedGraphics", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_renderOrderPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_renderOrderPriority", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sortOrderPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sortOrderPriority", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_blockingMask(
        &mut self,
        value: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_blockingMask", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_blockingObjects(
        &mut self,
        value: crate::UnityEngine::UI::GraphicRaycaster_BlockingObjects,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_blockingObjects", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ignoreReversedGraphics(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ignoreReversedGraphics", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+GraphicRaycaster")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::GraphicRaycaster {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
