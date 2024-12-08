#[cfg(feature = "BeatSaber+PerformancePresets+QuestPerformanceSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct QuestPerformanceSettings {
    __cordl_parent: crate::System::Object,
    pub _gpuLevel: crate::BeatSaber::PerformancePresets::ProcessorPerformanceLevel,
    pub _cpuLevel: crate::BeatSaber::PerformancePresets::ProcessorPerformanceLevel,
    pub _foveatedRenderingLevelMenu: crate::BeatSaber::PerformancePresets::FoveatedRenderingLevel,
    pub _foveatedRenderingLevelGameplay: crate::BeatSaber::PerformancePresets::FoveatedRenderingLevel,
    pub _eyeTrackedFoveatedRenderingEnabled: bool,
    pub _dynamicFoveatedRenderingEnabled: bool,
}
#[cfg(feature = "BeatSaber+PerformancePresets+QuestPerformanceSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::PerformancePresets::QuestPerformanceSettings =>
    "BeatSaber.PerformancePresets"."QuestPerformanceSettings"
);
#[cfg(feature = "BeatSaber+PerformancePresets+QuestPerformanceSettings")]
impl std::ops::Deref for crate::BeatSaber::PerformancePresets::QuestPerformanceSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+QuestPerformanceSettings")]
impl std::ops::DerefMut
for crate::BeatSaber::PerformancePresets::QuestPerformanceSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+QuestPerformanceSettings")]
impl crate::BeatSaber::PerformancePresets::QuestPerformanceSettings {
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
    pub fn get_cpuLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::PerformancePresets::ProcessorPerformanceLevel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::PerformancePresets::ProcessorPerformanceLevel = __cordl_object
            .invoke("get_cpuLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dynamicFoveatedRenderingEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_dynamicFoveatedRenderingEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_eyeTrackedFoveatedRenderingEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_eyeTrackedFoveatedRenderingEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_foveatedRenderingLevelGameplay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::PerformancePresets::FoveatedRenderingLevel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::PerformancePresets::FoveatedRenderingLevel = __cordl_object
            .invoke("get_foveatedRenderingLevelGameplay", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_foveatedRenderingLevelMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::PerformancePresets::FoveatedRenderingLevel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::PerformancePresets::FoveatedRenderingLevel = __cordl_object
            .invoke("get_foveatedRenderingLevelMenu", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gpuLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::PerformancePresets::ProcessorPerformanceLevel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::PerformancePresets::ProcessorPerformanceLevel = __cordl_object
            .invoke("get_gpuLevel", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+QuestPerformanceSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::PerformancePresets::QuestPerformanceSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
