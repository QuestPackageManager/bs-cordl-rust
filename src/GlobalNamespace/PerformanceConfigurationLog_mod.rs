#[cfg(feature = "PerformanceConfigurationLog")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformanceConfigurationLog {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PerformanceConfigurationLog")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PerformanceConfigurationLog =>
    ""."PerformanceConfigurationLog"
);
#[cfg(feature = "PerformanceConfigurationLog")]
impl std::ops::Deref for crate::GlobalNamespace::PerformanceConfigurationLog {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceConfigurationLog")]
impl std::ops::DerefMut for crate::GlobalNamespace::PerformanceConfigurationLog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceConfigurationLog")]
impl crate::GlobalNamespace::PerformanceConfigurationLog {
    pub fn Create(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        playerSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        recPlayState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecPlayBehaviour_State,
        >,
        stats: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PerformanceConfigurationStats,
        >,
        warning: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (settings, playerSettings, modifiers, recPlayState, stats, warning),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LogApplicationInfo(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogApplicationInfo", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogApplicationState(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogApplicationState", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogDynamicStates(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        stats: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PerformanceConfigurationStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogDynamicStates", (sb, stats))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogEntry<T>(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        category: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogEntry", (sb, category, name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogLevelSettings(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogLevelSettings", (sb, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogOculusXrInfo(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogOculusXrInfo", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogOpenXrInfo(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogOpenXrInfo", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogOvrInfo(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogOvrInfo", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogPlayerSettings(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        playerSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogPlayerSettings", (sb, playerSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogSettings(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        recPlayState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecPlayBehaviour_State,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogSettings", (sb, settings, recPlayState))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogSystemInfo(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogSystemInfo", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogXrInfo(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogXrInfo", (sb))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PerformanceConfigurationLog")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PerformanceConfigurationLog {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
