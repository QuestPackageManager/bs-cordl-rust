#[cfg(feature = "UnityEngine+EventSystems+Physics2DRaycaster")]
#[repr(C)]
#[derive(Debug)]
pub struct Physics2DRaycaster {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::PhysicsRaycaster,
    >,
    pub m_Hits: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::RaycastHit2D>,
    >,
}
#[cfg(feature = "UnityEngine+EventSystems+Physics2DRaycaster")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::Physics2DRaycaster =>
    "UnityEngine.EventSystems"."Physics2DRaycaster"
);
#[cfg(feature = "UnityEngine+EventSystems+Physics2DRaycaster")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::Physics2DRaycaster {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::PhysicsRaycaster,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+Physics2DRaycaster")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::Physics2DRaycaster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+Physics2DRaycaster")]
impl crate::UnityEngine::EventSystems::Physics2DRaycaster {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "UnityEngine+EventSystems+Physics2DRaycaster")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::Physics2DRaycaster {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
