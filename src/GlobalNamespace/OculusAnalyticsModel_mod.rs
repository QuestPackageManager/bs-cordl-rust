#[cfg(feature = "OculusAnalyticsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusAnalyticsModel {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "OculusAnalyticsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OculusAnalyticsModel => ""
    ."OculusAnalyticsModel"
);
#[cfg(feature = "OculusAnalyticsModel")]
impl std::ops::Deref for crate::GlobalNamespace::OculusAnalyticsModel {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusAnalyticsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusAnalyticsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusAnalyticsModel")]
impl crate::GlobalNamespace::OculusAnalyticsModel {
    pub fn LogClick(
        &mut self,
        clickType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        clickData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogClick", (clickType, clickData))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogEditAvatarEvent(
        &mut self,
        eventType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogEditAvatarEvent", (eventType, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogEvent(
        &mut self,
        eventType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogEvent", (eventType, eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogExposure(
        &mut self,
        exposureType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exposureData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogExposure", (exposureType, exposureData))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogImpression(
        &mut self,
        impressionType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        impressionData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogImpression", (impressionType, impressionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogSessionEvent(
        &mut self,
        eventType: crate::GlobalNamespace::BeatSaberSessionEventType,
        timeSinceSessionStart: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogSessionEvent", (eventType, timeSinceSessionStart))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OpenDataPrivacyPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenDataPrivacyPage", ())?;
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
    pub fn get_supportsOpenDataPrivacyPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_supportsOpenDataPrivacyPage", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusAnalyticsModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OculusAnalyticsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusAnalyticsModel")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IAnalyticsModel>>
for crate::GlobalNamespace::OculusAnalyticsModel {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IAnalyticsModel> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusAnalyticsModel")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IAnalyticsModel>>
for crate::GlobalNamespace::OculusAnalyticsModel {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IAnalyticsModel> {
        unsafe { std::mem::transmute(self) }
    }
}
