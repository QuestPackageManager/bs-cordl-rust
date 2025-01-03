#[cfg(feature = "VRUIControls+VRGraphicRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct VRGraphicRaycaster {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseRaycaster,
    pub _blockingMask: crate::UnityEngine::LayerMask,
    pub _physicsRaycaster: quest_hook::libil2cpp::Gc<
        crate::VRUIControls::PhysicsRaycasterWithCache,
    >,
    pub _canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    pub _raycastResults: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult,
        >,
    >,
    pub _curvedCanvasSettingsHelper: quest_hook::libil2cpp::Gc<
        crate::HMUI::CurvedCanvasSettingsHelper,
    >,
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        resultAppendList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::EventSystems::RaycastResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Raycast", (eventData, resultAppendList))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastCanvas(
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        ray: crate::UnityEngine::Ray,
        hitDistance: f32,
        curvedUIRadius: f32,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::VRUIControls::VRGraphicRaycaster_VRGraphicRaycastResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RaycastCanvas",
                (canvas, ray, hitDistance, curvedUIRadius, results),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eventCamera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = __cordl_object
            .invoke("get_eventCamera", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "VRUIControls+VRGraphicRaycaster+VRGraphicRaycastResult")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct VRGraphicRaycaster_VRGraphicRaycastResult {
    pub graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
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
        graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
        distance: f32,
        position: crate::UnityEngine::Vector3,
        insideRootCanvasPosition: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (graphic, distance, position, insideRootCanvasPosition),
        )?;
        Ok(__cordl_ret.into())
    }
}
