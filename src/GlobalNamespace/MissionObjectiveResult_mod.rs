#[cfg(feature = "MissionObjectiveResult")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionObjectiveResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _missionObjective_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionObjective,
    >,
    pub _cleared_k__BackingField: bool,
    pub _value_k__BackingField: i32,
}
#[cfg(feature = "MissionObjectiveResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionObjectiveResult => ""
    ."MissionObjectiveResult"
);
#[cfg(feature = "MissionObjectiveResult")]
impl std::ops::Deref for crate::GlobalNamespace::MissionObjectiveResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveResult")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionObjectiveResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveResult")]
impl crate::GlobalNamespace::MissionObjectiveResult {
    pub fn New(
        missionObjective: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionObjective,
        >,
        cleared: bool,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (missionObjective, cleared, value))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        missionObjective: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionObjective,
        >,
        cleared: bool,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (missionObjective, cleared, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cleared(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_cleared", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_missionObjective(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjective>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionObjective,
        > = __cordl_object.invoke("get_missionObjective", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cleared(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cleared", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_missionObjective(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjective>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_missionObjective", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_value(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_value", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionObjectiveResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionObjectiveResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
