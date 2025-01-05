#[cfg(feature = "HoudiniEngineUnity+HEU_BakedDataEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_BakedDataEvent {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_BakedEventData>,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_BakedDataEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_BakedDataEvent =>
    "HoudiniEngineUnity"."HEU_BakedDataEvent"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_BakedDataEvent")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_BakedDataEvent {
    type Target = quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_BakedEventData>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_BakedDataEvent")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_BakedDataEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_BakedDataEvent")]
impl crate::HoudiniEngineUnity::HEU_BakedDataEvent {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_BakedDataEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_BakedDataEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
