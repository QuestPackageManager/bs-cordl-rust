#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackedDeviceRaycaster {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::BaseRaycaster,
    >,
    pub m_RaycastResultsCache: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster_RaycastHitData,
    >,
    pub m_IgnoreReversedGraphics: bool,
    pub m_CheckFor2DOcclusion: bool,
    pub m_CheckFor3DOcclusion: bool,
    pub m_MaxDistance: f32,
    pub m_BlockingMask: crate::UnityEngine::LayerMask,
    pub m_Canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster =>
    "UnityEngine.InputSystem.UI"."TrackedDeviceRaycaster"
);
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::BaseRaycaster,
    >;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
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
    pub fn PerformRaycast(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::UI::ExtendedPointerEventData,
        >,
        resultAppendList: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::RaycastResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformRaycast", (eventData, resultAppendList))?;
        Ok(__cordl_ret.into())
    }
    pub fn RayIntersectsRectTransform(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        ray: crate::UnityEngine::Ray,
        worldPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        distance: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RayIntersectsRectTransform",
                (transform, ray, worldPosition, distance),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        resultAppendList: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::RaycastResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Raycast", (eventData, resultAppendList))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortedRaycastGraphics(
        &mut self,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::UI::TrackedDeviceRaycaster_RaycastHitData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortedRaycastGraphics", (canvas, ray, results))?;
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
    pub fn get_blockingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("get_blockingMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canvas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas> = __cordl_object
            .invoke("get_canvas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_checkFor2DOcclusion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_checkFor2DOcclusion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_checkFor3DOcclusion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_checkFor3DOcclusion", ())?;
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
    pub fn get_ignoreReversedGraphics(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ignoreReversedGraphics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxDistance", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+InputSystem+UI+TrackedDeviceRaycaster+RaycastHitData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TrackedDeviceRaycaster_RaycastHitData {
    pub _graphic_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Graphic,
    >,
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
    pub fn _ctor(
        &mut self,
        graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
        worldHitPosition: crate::UnityEngine::Vector3,
        screenPosition: crate::UnityEngine::Vector2,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (graphic, worldHitPosition, screenPosition, distance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_distance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_distance",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_graphic",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_screenPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_screenPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldHitPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_worldHitPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
