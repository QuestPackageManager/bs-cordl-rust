#[cfg(feature = "GameSessionAnalyticsManager")]
#[repr(C)]
#[derive(Debug)]
pub struct GameSessionAnalyticsManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _analyticsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAnalyticsModel,
    >,
    pub _platformInit: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::IPlatformInit>,
    pub lastSessionStartTime: f32,
    pub _initialized: bool,
    pub _initializationTask: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task,
    >,
}
#[cfg(feature = "GameSessionAnalyticsManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameSessionAnalyticsManager =>
    ""."GameSessionAnalyticsManager"
);
#[cfg(feature = "GameSessionAnalyticsManager")]
impl std::ops::Deref for crate::GlobalNamespace::GameSessionAnalyticsManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameSessionAnalyticsManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameSessionAnalyticsManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameSessionAnalyticsManager")]
impl crate::GlobalNamespace::GameSessionAnalyticsManager {
    pub fn LogSessionEventAfterPlatformInit(
        &mut self,
        hasFocus: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("LogSessionEventAfterPlatformInit", (hasFocus))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogSessionEventOnFocus(
        &mut self,
        hasFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogSessionEventOnFocus", (hasFocus))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogSessionFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogSessionFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LogSessionStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogSessionStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnApplicationFocus(
        &mut self,
        hasFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationFocus", (hasFocus))?;
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
#[cfg(feature = "GameSessionAnalyticsManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameSessionAnalyticsManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
