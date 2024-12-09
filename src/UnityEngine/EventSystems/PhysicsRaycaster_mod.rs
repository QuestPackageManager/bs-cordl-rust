#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct PhysicsRaycaster {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseRaycaster,
    pub m_EventCamera: *mut crate::UnityEngine::Camera,
    pub m_EventMask: crate::UnityEngine::LayerMask,
    pub m_MaxRayIntersections: i32,
    pub m_LastMaxRayIntersections: i32,
    pub m_Hits: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit>,
}
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::PhysicsRaycaster =>
    "UnityEngine.EventSystems"."PhysicsRaycaster"
);
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::PhysicsRaycaster {
    type Target = crate::UnityEngine::EventSystems::BaseRaycaster;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::PhysicsRaycaster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster")]
impl crate::UnityEngine::EventSystems::PhysicsRaycaster {
    pub const kNoEventMaskSet: i32 = -1i32;
    #[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster+RaycastHitComparer")]
    pub type RaycastHitComparer = crate::UnityEngine::EventSystems::PhysicsRaycaster_RaycastHitComparer;
    pub fn ComputeRayAndDistance(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
        ray: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
        eventDisplayIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        distanceToClipPlane: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ComputeRayAndDistance",
                (eventData, ray, eventDisplayIndex, distanceToClipPlane),
            )?;
        Ok(__cordl_ret)
    }
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
    pub fn get_depth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_depth", ())?;
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
    pub fn get_eventMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("get_eventMask", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_finalEventMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_finalEventMask", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxRayIntersections(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxRayIntersections", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_eventMask(
        &mut self,
        value: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_eventMask", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_maxRayIntersections(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxRayIntersections", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::PhysicsRaycaster {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster+RaycastHitComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct PhysicsRaycaster_RaycastHitComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster+RaycastHitComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::EventSystems::PhysicsRaycaster_RaycastHitComparer =>
    "UnityEngine.EventSystems"."PhysicsRaycaster/RaycastHitComparer"
);
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster+RaycastHitComparer")]
impl std::ops::Deref
for crate::UnityEngine::EventSystems::PhysicsRaycaster_RaycastHitComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster+RaycastHitComparer")]
impl std::ops::DerefMut
for crate::UnityEngine::EventSystems::PhysicsRaycaster_RaycastHitComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster+RaycastHitComparer")]
impl crate::UnityEngine::EventSystems::PhysicsRaycaster_RaycastHitComparer {
    pub fn Compare(
        &mut self,
        x: crate::UnityEngine::RaycastHit,
        y: crate::UnityEngine::RaycastHit,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+EventSystems+PhysicsRaycaster+RaycastHitComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::PhysicsRaycaster_RaycastHitComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
