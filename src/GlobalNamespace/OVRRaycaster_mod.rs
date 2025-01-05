#[cfg(feature = "OVRRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRRaycaster {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::GraphicRaycaster>,
    pub pointer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub sortOrder: i32,
    pub m_Canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    pub m_RaycastResults: quest_hook::libil2cpp::Gc<
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
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::GraphicRaycaster>;
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
    pub fn GetScreenPosition(
        &mut self,
        raycastResult: crate::UnityEngine::EventSystems::RaycastResult,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetScreenPosition", (raycastResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn GraphicRaycast(
        &mut self,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        ray: crate::UnityEngine::Ray,
        results: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRRaycaster_RaycastHit,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GraphicRaycast", (canvas, ray, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFocussed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsFocussed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnPointerEnter(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::PointerEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerEnter", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn RayIntersectsRectTransform(
        rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        ray: crate::UnityEngine::Ray,
        worldPos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RayIntersectsRectTransform", (rectTransform, ray, worldPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastPointer(
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
            .invoke("RaycastPointer", (eventData, resultAppendList))?;
        Ok(__cordl_ret.into())
    }
    pub fn Raycast_Gc_Gc1(
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
    pub fn Raycast_Ray__cordl_bool0(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        resultAppendList: quest_hook::libil2cpp::Gc<
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
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn get_sortOrderPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sortOrderPriority", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "OVRRaycaster")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEventSystemHandler>,
> for crate::GlobalNamespace::OVRRaycaster {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IEventSystemHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRRaycaster")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEventSystemHandler>,
> for crate::GlobalNamespace::OVRRaycaster {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IEventSystemHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRRaycaster")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerEnterHandler>,
> for crate::GlobalNamespace::OVRRaycaster {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IPointerEnterHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRRaycaster")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerEnterHandler>,
> for crate::GlobalNamespace::OVRRaycaster {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IPointerEnterHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRRaycaster+RaycastHit")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRRaycaster_RaycastHit {
    pub graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
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
