#[cfg(feature = "MissionObjective")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionObjective {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _type: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjectiveTypeSO>,
    pub _referenceValueComparisonType: crate::GlobalNamespace::MissionObjective_ReferenceValueComparisonType,
    pub _referenceValue: i32,
}
#[cfg(feature = "MissionObjective")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionObjective => ""
    ."MissionObjective"
);
#[cfg(feature = "MissionObjective")]
impl std::ops::Deref for crate::GlobalNamespace::MissionObjective {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjective")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionObjective {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjective")]
impl crate::GlobalNamespace::MissionObjective {
    #[cfg(feature = "MissionObjective+ReferenceValueComparisonType")]
    pub type ReferenceValueComparisonType = crate::GlobalNamespace::MissionObjective_ReferenceValueComparisonType;
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
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
    pub fn get_referenceValue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_referenceValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_referenceValueComparisonType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MissionObjective_ReferenceValueComparisonType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MissionObjective_ReferenceValueComparisonType = __cordl_object
            .invoke("get_referenceValueComparisonType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjectiveTypeSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionObjectiveTypeSO,
        > = __cordl_object.invoke("get_type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        obj1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjective>,
        obj2: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjective>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (obj1, obj2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        obj1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjective>,
        obj2: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjective>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (obj1, obj2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionObjective")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MissionObjective {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MissionObjective+ReferenceValueComparisonType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionObjective_ReferenceValueComparisonType {
    Equal = 1i32,
    Max = 2i32,
    Min = 3i32,
    None = 0i32,
}
#[cfg(feature = "MissionObjective+ReferenceValueComparisonType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionObjective_ReferenceValueComparisonType => ""
    ."MissionObjective/ReferenceValueComparisonType"
);
