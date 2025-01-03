#[cfg(feature = "HandsMovementMissionObjectiveChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct HandsMovementMissionObjectiveChecker {
    __cordl_parent: crate::GlobalNamespace::SimpleValueMissionObjectiveChecker,
    pub _saberActivityCounter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SaberActivityCounter,
    >,
}
#[cfg(feature = "HandsMovementMissionObjectiveChecker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::HandsMovementMissionObjectiveChecker => ""
    ."HandsMovementMissionObjectiveChecker"
);
#[cfg(feature = "HandsMovementMissionObjectiveChecker")]
impl std::ops::Deref for crate::GlobalNamespace::HandsMovementMissionObjectiveChecker {
    type Target = crate::GlobalNamespace::SimpleValueMissionObjectiveChecker;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HandsMovementMissionObjectiveChecker")]
impl std::ops::DerefMut
for crate::GlobalNamespace::HandsMovementMissionObjectiveChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HandsMovementMissionObjectiveChecker")]
impl crate::GlobalNamespace::HandsMovementMissionObjectiveChecker {
    pub fn HandleTotalDistanceDidChange(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleTotalDistanceDidChange", (distance))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
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
#[cfg(feature = "HandsMovementMissionObjectiveChecker")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HandsMovementMissionObjectiveChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
