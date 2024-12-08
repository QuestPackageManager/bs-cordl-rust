#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster+RaycastHitData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TrackedDeviceRaycaster_RaycastHitData {
    pub _graphic_k__BackingField: *mut crate::UnityEngine::UI::Graphic,
    pub _worldHitPosition_k__BackingField: crate::UnityEngine::Vector3,
    pub _screenPosition_k__BackingField: crate::UnityEngine::Vector2,
    pub _distance_k__BackingField: f32,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster+RaycastHitData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster_RaycastHitData =>
    "UnityEngine.InputSystem.UI"."TrackedDeviceRaycaster/RaycastHitData"
);
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster+RaycastHitData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster_RaycastHitData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster+RaycastHitData")]
impl crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster_RaycastHitData {
    pub fn get_worldHitPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_worldHitPosition",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        graphic: *mut crate::UnityEngine::UI::Graphic,
        worldHitPosition: crate::UnityEngine::Vector3,
        screenPosition: crate::UnityEngine::Vector2,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (graphic, worldHitPosition, screenPosition, distance),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_screenPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_screenPosition",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_distance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_distance",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_graphic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Graphic> {
        let __cordl_ret: *mut crate::UnityEngine::UI::Graphic = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_graphic",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackedDeviceRaycaster {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseRaycaster,
    pub m_RaycastResultsCache: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster_RaycastHitData,
    >,
    pub m_IgnoreReversedGraphics: bool,
    pub m_CheckFor2DOcclusion: bool,
    pub m_CheckFor3DOcclusion: bool,
    pub m_MaxDistance: f32,
    pub m_BlockingMask: crate::UnityEngine::LayerMask,
    pub m_Canvas: *mut crate::UnityEngine::Canvas,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster =>
    "UnityEngine.InputSystem.UI"."TrackedDeviceRaycaster"
);
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster {
    type Target = crate::UnityEngine::EventSystems::BaseRaycaster;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster")]
impl crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster {
    #[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster+RaycastHitData")]
    pub type RaycastHitData = crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster_RaycastHitData;
    #[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster+__c")]
    pub type __c = crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster___c;
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxDistance", ())?;
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
    pub fn PerformRaycast(
        &mut self,
        eventData: *mut crate::UnityEngine::InputSystem::UI::ExtendedPointerEventData,
        resultAppendList: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::EventSystems::RaycastResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformRaycast", (eventData, resultAppendList))?;
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
    pub fn get_checkFor3DOcclusion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_checkFor3DOcclusion", ())?;
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
    pub fn set_checkFor2DOcclusion(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_checkFor2DOcclusion", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_checkFor3DOcclusion(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_checkFor3DOcclusion", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_checkFor2DOcclusion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_checkFor2DOcclusion", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_maxDistance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxDistance", (value))?;
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
    pub fn SortedRaycastGraphics(
        &mut self,
        canvas: *mut crate::UnityEngine::Canvas,
        ray: crate::UnityEngine::Ray,
        results: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster_RaycastHitData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortedRaycastGraphics", (canvas, ray, results))?;
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
    pub fn get_ignoreReversedGraphics(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ignoreReversedGraphics", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
