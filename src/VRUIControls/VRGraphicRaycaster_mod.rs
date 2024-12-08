#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VRGraphicRaycaster_VRGraphicRaycastResult {
    pub graphic: *mut crate::UnityEngine::UI::Graphic,
    pub distance: f32,
    pub position: crate::UnityEngine::Vector3,
    pub insideRootCanvasPosition: crate::UnityEngine::Vector2,
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult => "VRUIControls"
    ."VRGraphicRaycaster/VRGraphicRaycastResult"
);
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
impl crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult {
    pub fn _ctor(
        &mut self,
        graphic: *mut crate::UnityEngine::UI::Graphic,
        distance: f32,
        position: crate::UnityEngine::Vector3,
        insideRootCanvasPosition: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (graphic, distance, position, insideRootCanvasPosition),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct VRGraphicRaycaster {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseRaycaster,
    pub _blockingMask: crate::UnityEngine::LayerMask,
    pub _physicsRaycaster: *mut crate::VRUIControls::PhysicsRaycasterWithCache,
    pub _canvas: *mut crate::UnityEngine::Canvas,
    pub _raycastResults: *mut crate::System::Collections::Generic::List_1<
        crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult,
    >,
    pub _curvedCanvasSettingsHelper: *mut crate::HMUI::CurvedCanvasSettingsHelper,
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::VRUIControls::VRGraphicRaycaster =>
    "VRUIControls"."VRGraphicRaycaster"
);
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
impl std::ops::Deref for crate::VRUIControls::VRGraphicRaycaster {
    type Target = crate::UnityEngine::EventSystems::BaseRaycaster;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
impl std::ops::DerefMut for crate::VRUIControls::VRGraphicRaycaster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
impl crate::VRUIControls::VRGraphicRaycaster {
    pub const kPhysics3DRaycastDistance: f32 = 6f32;
    #[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
    pub type VRGraphicRaycastResult = crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult;
    #[cfg(feature = "VRUIControls+VRGraphicRaycaster+__c")]
    pub type __c = crate::VRUIControls::VRGraphicRaycaster___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
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
}
#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
impl quest_hook::libil2cpp::ObjectType for crate::VRUIControls::VRGraphicRaycaster {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
