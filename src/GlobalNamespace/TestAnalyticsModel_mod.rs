#[cfg(feature = "TestAnalyticsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct TestAnalyticsModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TestAnalyticsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TestAnalyticsModel => ""
    ."TestAnalyticsModel"
);
#[cfg(feature = "TestAnalyticsModel")]
impl std::ops::Deref for crate::GlobalNamespace::TestAnalyticsModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TestAnalyticsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::TestAnalyticsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TestAnalyticsModel")]
impl crate::GlobalNamespace::TestAnalyticsModel {
    pub fn LogClick(
        &mut self,
        clickType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        clickData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
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
        avatarEventData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogEditAvatarEvent", (eventType, avatarEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogEvent(
        &mut self,
        eventType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
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
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
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
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
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
        _cordl__: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogSessionEvent", (eventType, _cordl__))?;
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
#[cfg(feature = "TestAnalyticsModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TestAnalyticsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TestAnalyticsModel")]
impl AsRef<crate::GlobalNamespace::IAnalyticsModel>
for crate::GlobalNamespace::TestAnalyticsModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IAnalyticsModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TestAnalyticsModel")]
impl AsMut<crate::GlobalNamespace::IAnalyticsModel>
for crate::GlobalNamespace::TestAnalyticsModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IAnalyticsModel {
        unsafe { std::mem::transmute(self) }
    }
}
