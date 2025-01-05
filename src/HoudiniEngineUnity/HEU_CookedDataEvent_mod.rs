#[cfg(feature = "HoudiniEngineUnity+HEU_CookedDataEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_CookedDataEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_CookedEventData>,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_CookedDataEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_CookedDataEvent =>
    "HoudiniEngineUnity"."HEU_CookedDataEvent"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_CookedDataEvent")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_CookedDataEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_CookedEventData>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_CookedDataEvent")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_CookedDataEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_CookedDataEvent")]
impl crate::HoudiniEngineUnity::HEU_CookedDataEvent {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_CookedDataEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_CookedDataEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
