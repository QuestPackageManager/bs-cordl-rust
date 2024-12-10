#[cfg(feature = "UnityEngine+UIElements+PointerStationaryEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerStationaryEvent {
    __cordl_parent: crate::UnityEngine::UIElements::PointerEventBase_1<
        *mut crate::UnityEngine::UIElements::PointerStationaryEvent,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+PointerStationaryEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PointerStationaryEvent
    => "UnityEngine.UIElements"."PointerStationaryEvent"
);
#[cfg(feature = "UnityEngine+UIElements+PointerStationaryEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerStationaryEvent {
    type Target = crate::UnityEngine::UIElements::PointerEventBase_1<
        *mut crate::UnityEngine::UIElements::PointerStationaryEvent,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerStationaryEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerStationaryEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerStationaryEvent")]
impl crate::UnityEngine::UIElements::PointerStationaryEvent {
    #[cfg(feature = "UnityEngine+UIElements+PointerStationaryEvent+__c")]
    pub type __c = crate::UnityEngine::UIElements::PointerStationaryEvent___c;
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LocalInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "UnityEngine+UIElements+PointerStationaryEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PointerStationaryEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
