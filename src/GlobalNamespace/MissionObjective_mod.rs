#[cfg(feature = "MissionObjective")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionObjective {
    __cordl_parent: crate::System::Object,
    pub _type: *mut MissionObjectiveTypeSO,
    pub _referenceValueComparisonType: crate::GlobalNamespace::MissionObjective_ReferenceValueComparisonType,
    pub _referenceValue: i32,
}
#[cfg(feature = "MissionObjective")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionObjective => ""."MissionObjective"
);
#[cfg(feature = "MissionObjective")]
impl std::ops::Deref for MissionObjective {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjective")]
impl std::ops::DerefMut for MissionObjective {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjective")]
impl MissionObjective {
    #[cfg(feature = "MissionObjective+ReferenceValueComparisonType")]
    pub type ReferenceValueComparisonType = crate::GlobalNamespace::MissionObjective_ReferenceValueComparisonType;
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
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
    pub fn get_referenceValue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_referenceValue", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionObjectiveTypeSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionObjectiveTypeSO = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionObjective")]
impl quest_hook::libil2cpp::ObjectType for MissionObjective {
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
