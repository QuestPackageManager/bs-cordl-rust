#[cfg(feature = "IAnalyticsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct IAnalyticsModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IAnalyticsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IAnalyticsModel => ""."IAnalyticsModel"
);
#[cfg(feature = "IAnalyticsModel")]
impl std::ops::Deref for IAnalyticsModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IAnalyticsModel")]
impl std::ops::DerefMut for IAnalyticsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IAnalyticsModel")]
impl IAnalyticsModel {
    pub fn LogClick(
        &mut self,
        clickType: *mut crate::System::String,
        clickData: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogClick", (clickType, clickData))?;
        Ok(__cordl_ret)
    }
    pub fn LogEditAvatarEvent(
        &mut self,
        eventType: *mut crate::System::String,
        eventData: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogEditAvatarEvent", (eventType, eventData))?;
        Ok(__cordl_ret)
    }
    pub fn LogEvent(
        &mut self,
        eventType: *mut crate::System::String,
        eventData: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogEvent", (eventType, eventData))?;
        Ok(__cordl_ret)
    }
    pub fn LogExposure(
        &mut self,
        exposureType: *mut crate::System::String,
        exposureData: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogExposure", (exposureType, exposureData))?;
        Ok(__cordl_ret)
    }
    pub fn LogImpression(
        &mut self,
        impressionType: *mut crate::System::String,
        impressionData: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogImpression", (impressionType, impressionData))?;
        Ok(__cordl_ret)
    }
    pub fn OpenDataPrivacyPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenDataPrivacyPage", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_supportsOpenDataPrivacyPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_supportsOpenDataPrivacyPage", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IAnalyticsModel")]
impl quest_hook::libil2cpp::ObjectType for IAnalyticsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
