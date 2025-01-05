#[cfg(feature = "OculusTestExperiment")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusTestExperiment {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _experimentModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IExperimentModel,
    >,
}
#[cfg(feature = "OculusTestExperiment")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OculusTestExperiment => ""
    ."OculusTestExperiment"
);
#[cfg(feature = "OculusTestExperiment")]
impl std::ops::Deref for crate::GlobalNamespace::OculusTestExperiment {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusTestExperiment")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusTestExperiment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusTestExperiment")]
impl crate::GlobalNamespace::OculusTestExperiment {
    pub const kIsInTest1Key: &'static str = "beatsaber_experiments:test_parameter";
    #[cfg(feature = "OculusTestExperiment+ExperimentData")]
    pub type ExperimentData = crate::GlobalNamespace::OculusTestExperiment_ExperimentData;
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
#[cfg(feature = "OculusTestExperiment")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OculusTestExperiment {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusTestExperiment+ExperimentData")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusTestExperiment_ExperimentData {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _experimentPlatformKey: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "OculusTestExperiment+ExperimentData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusTestExperiment_ExperimentData => ""
    ."OculusTestExperiment/ExperimentData"
);
#[cfg(feature = "OculusTestExperiment+ExperimentData")]
impl std::ops::Deref for crate::GlobalNamespace::OculusTestExperiment_ExperimentData {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusTestExperiment+ExperimentData")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusTestExperiment_ExperimentData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusTestExperiment+ExperimentData")]
impl crate::GlobalNamespace::OculusTestExperiment_ExperimentData {
    pub fn New(
        experimentPlatformKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (experimentPlatformKey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        experimentPlatformKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (experimentPlatformKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_experimentPlatformKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_experimentPlatformKey", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusTestExperiment+ExperimentData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusTestExperiment_ExperimentData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusTestExperiment+ExperimentData")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IExperimentData>>
for crate::GlobalNamespace::OculusTestExperiment_ExperimentData {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IExperimentData> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusTestExperiment+ExperimentData")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IExperimentData>>
for crate::GlobalNamespace::OculusTestExperiment_ExperimentData {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IExperimentData> {
        unsafe { std::mem::transmute(self) }
    }
}
