#[cfg(feature = "OculusDeeplinkManager")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusDeeplinkManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub didReceiveDeeplinkEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Deeplink>,
        >,
    >,
    pub _currentDeeplink: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Deeplink>,
    pub _oculusPlatformWasInitialized: bool,
}
#[cfg(feature = "OculusDeeplinkManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusDeeplinkManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusDeeplinkManager";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OculusDeeplinkManager")]
impl std::ops::Deref for crate::GlobalNamespace::OculusDeeplinkManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn IsAtLeastOneFieldPopulated(
        &mut self,
        deeplink: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Deeplink>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsAtLeastOneFieldPopulated", (deeplink))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OculusPlatformWasInitialized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OculusPlatformWasInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetJoinIntentReceivedNotificationCallback(
        &mut self,
        message: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::GroupPresenceJoinIntent,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetJoinIntentReceivedNotificationCallback", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDeeplinkMessage(
        &mut self,
        joinIntent: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::GroupPresenceJoinIntent,
        >,
        launchDetails: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchDetails,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDeeplinkMessage", (joinIntent, launchDetails))?;
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
    pub fn add_didReceiveDeeplinkEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Deeplink>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didReceiveDeeplinkEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentDeeplink(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Deeplink>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Deeplink> = __cordl_object
            .invoke("get_currentDeeplink", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didReceiveDeeplinkEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::Deeplink>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didReceiveDeeplinkEvent", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "OculusDeeplinkManager")]
impl AsRef<crate::GlobalNamespace::IDeeplinkManager>
for crate::GlobalNamespace::OculusDeeplinkManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::IDeeplinkManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusDeeplinkManager")]
impl AsMut<crate::GlobalNamespace::IDeeplinkManager>
for crate::GlobalNamespace::OculusDeeplinkManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IDeeplinkManager {
        unsafe { std::mem::transmute(self) }
    }
}
