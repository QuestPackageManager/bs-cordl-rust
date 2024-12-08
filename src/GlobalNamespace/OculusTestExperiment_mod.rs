#[cfg(feature = "OculusTestExperiment+ExperimentData")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusTestExperiment_ExperimentData {
    __cordl_parent: crate::System::Object,
    pub _experimentPlatformKey: *mut crate::System::String,
}
#[cfg(feature = "OculusTestExperiment+ExperimentData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusTestExperiment_ExperimentData => ""
    ."OculusTestExperiment/ExperimentData"
);
#[cfg(feature = "OculusTestExperiment+ExperimentData")]
impl std::ops::Deref for crate::GlobalNamespace::OculusTestExperiment_ExperimentData {
    type Target = crate::System::Object;
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
        experimentPlatformKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (experimentPlatformKey))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        experimentPlatformKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (experimentPlatformKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_experimentPlatformKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_experimentPlatformKey", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "OculusTestExperiment")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusTestExperiment {
    __cordl_parent: crate::System::Object,
    pub _experimentModel: *mut crate::GlobalNamespace::IExperimentModel,
}
#[cfg(feature = "OculusTestExperiment")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OculusTestExperiment => ""
    ."OculusTestExperiment"
);
#[cfg(feature = "OculusTestExperiment")]
impl std::ops::Deref for crate::GlobalNamespace::OculusTestExperiment {
    type Target = crate::System::Object;
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
    #[cfg(feature = "OculusTestExperiment+_Init_d__3")]
    pub type _Init_d__3 = crate::GlobalNamespace::OculusTestExperiment__Init_d__3;
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
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
