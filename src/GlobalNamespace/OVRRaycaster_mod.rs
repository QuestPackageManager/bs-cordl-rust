#[cfg(feature = "OVRRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRRaycaster {
    __cordl_parent: crate::UnityEngine::UI::GraphicRaycaster,
    pub pointer: *mut crate::UnityEngine::GameObject,
    pub sortOrder: i32,
    pub m_Canvas: *mut crate::UnityEngine::Canvas,
    pub m_RaycastResults: *mut crate::System::Collections::Generic::List_1<
        crate::GlobalNamespace::OVRRaycaster_RaycastHit,
    >,
}
#[cfg(feature = "OVRRaycaster")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRRaycaster => ""
    ."OVRRaycaster"
);
#[cfg(feature = "OVRRaycaster")]
impl std::ops::Deref for crate::GlobalNamespace::OVRRaycaster {
    type Target = crate::UnityEngine::UI::GraphicRaycaster;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRRaycaster")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRRaycaster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRRaycaster")]
impl crate::GlobalNamespace::OVRRaycaster {
    #[cfg(feature = "OVRRaycaster+RaycastHit")]
    pub type RaycastHit = crate::GlobalNamespace::OVRRaycaster_RaycastHit;
    #[cfg(feature = "OVRRaycaster+__c")]
    pub type __c = crate::GlobalNamespace::OVRRaycaster___c;
    pub fn GetScreenPosition(
        &mut self,
        raycastResult: crate::UnityEngine::EventSystems::RaycastResult,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetScreenPosition", (raycastResult))?;
        Ok(__cordl_ret)
    }
    pub fn GraphicRaycast(
        &mut self,
        canvas: *mut crate::UnityEngine::Canvas,
        ray: crate::UnityEngine::Ray,
        results: *mut crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::OVRRaycaster_RaycastHit,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GraphicRaycast", (canvas, ray, results))?;
        Ok(__cordl_ret)
    }
    pub fn IsFocussed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsFocussed", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnPointerEnter(
        &mut self,
        e: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerEnter", (e))?;
        Ok(__cordl_ret)
    }
    pub fn RaycastPointer(
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
            .invoke("RaycastPointer", (eventData, resultAppendList))?;
        Ok(__cordl_ret)
    }
    pub fn Raycast_PointerEventData_List_1_1(
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
    pub fn Raycast_Ray__cordl_bool0(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
        resultAppendList: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::EventSystems::RaycastResult,
        >,
        ray: crate::UnityEngine::Ray,
        checkForBlocking: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Raycast", (eventData, resultAppendList, ray, checkForBlocking))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn get_sortOrderPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sortOrderPriority", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRRaycaster")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRRaycaster {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRRaycaster+RaycastHit")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRRaycaster_RaycastHit {
    pub graphic: *mut crate::UnityEngine::UI::Graphic,
    pub worldPos: crate::UnityEngine::Vector3,
    pub fromMouse: bool,
}
#[cfg(feature = "OVRRaycaster+RaycastHit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRRaycaster_RaycastHit => ""
    ."OVRRaycaster/RaycastHit"
);
#[cfg(feature = "OVRRaycaster+RaycastHit")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRRaycaster_RaycastHit {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRRaycaster+RaycastHit")]
impl crate::GlobalNamespace::OVRRaycaster_RaycastHit {}
