#[cfg(feature = "OculusDeeplinkManager")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusDeeplinkManager {
    __cordl_parent: crate::System::Object,
    pub didReceiveDeeplinkEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::Deeplink,
    >,
    pub _currentDeeplink: *mut crate::GlobalNamespace::Deeplink,
    pub _oculusPlatformWasInitialized: bool,
}
#[cfg(feature = "OculusDeeplinkManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OculusDeeplinkManager => ""
    ."OculusDeeplinkManager"
);
#[cfg(feature = "OculusDeeplinkManager")]
impl std::ops::Deref for crate::GlobalNamespace::OculusDeeplinkManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusDeeplinkManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusDeeplinkManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusDeeplinkManager")]
impl crate::GlobalNamespace::OculusDeeplinkManager {
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
    pub fn IsAtLeastOneFieldPopulated(
        &mut self,
        deeplink: *mut crate::GlobalNamespace::Deeplink,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsAtLeastOneFieldPopulated", (deeplink))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OculusPlatformWasInitialized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OculusPlatformWasInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetJoinIntentReceivedNotificationCallback(
        &mut self,
        message: *mut crate::Oculus::Platform::Message_1<
            *mut crate::Oculus::Platform::Models::GroupPresenceJoinIntent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetJoinIntentReceivedNotificationCallback", (message))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDeeplinkMessage(
        &mut self,
        joinIntent: *mut crate::Oculus::Platform::Models::GroupPresenceJoinIntent,
        launchDetails: *mut crate::Oculus::Platform::Models::LaunchDetails,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDeeplinkMessage", (joinIntent, launchDetails))?;
        Ok(__cordl_ret)
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
    pub fn add_didReceiveDeeplinkEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::Deeplink>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didReceiveDeeplinkEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentDeeplink(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::Deeplink> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::Deeplink = __cordl_object
            .invoke("get_currentDeeplink", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didReceiveDeeplinkEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::GlobalNamespace::Deeplink>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didReceiveDeeplinkEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OculusDeeplinkManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusDeeplinkManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
