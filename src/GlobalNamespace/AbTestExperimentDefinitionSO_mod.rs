#[cfg(feature = "AbTestExperimentDefinitionSO")]
#[repr(C)]
#[derive(Debug)]
pub struct AbTestExperimentDefinitionSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >,
    pub _experimentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _test1GroupSize: f32,
    pub _test2GroupSize: f32,
    pub _controlGroupSize: f32,
    pub _salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _currentUserTreatmentGroup: crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group,
}
#[cfg(feature = "AbTestExperimentDefinitionSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AbTestExperimentDefinitionSO =>
    ""."AbTestExperimentDefinitionSO"
);
#[cfg(feature = "AbTestExperimentDefinitionSO")]
impl std::ops::Deref for crate::GlobalNamespace::AbTestExperimentDefinitionSO {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AbTestExperimentDefinitionSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::AbTestExperimentDefinitionSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AbTestExperimentDefinitionSO")]
impl crate::GlobalNamespace::AbTestExperimentDefinitionSO {
    #[cfg(feature = "AbTestExperimentDefinitionSO+Group")]
    pub type Group = crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group;
    pub fn AbSplit(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group = __cordl_object
            .invoke("AbSplit", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeCurrentUserTreatment(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeCurrentUserTreatment", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForceSetTreatmentGroup(
        &mut self,
        group: crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceSetTreatmentGroup", (group))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
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
    pub fn get_currentUserTreatmentGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AbTestExperimentDefinitionSO_Group = __cordl_object
            .invoke("get_currentUserTreatmentGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_experimentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_experimentName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_test1GroupSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_test1GroupSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_test2GroupSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_test2GroupSize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AbTestExperimentDefinitionSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AbTestExperimentDefinitionSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AbTestExperimentDefinitionSO+Group")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AbTestExperimentDefinitionSO_Group {
    #[default]
    Control = 0i32,
    Test1 = 1i32,
    Test2 = 2i32,
}
#[cfg(feature = "AbTestExperimentDefinitionSO+Group")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AbTestExperimentDefinitionSO_Group => ""
    ."AbTestExperimentDefinitionSO/Group"
);
