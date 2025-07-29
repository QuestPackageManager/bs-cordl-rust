#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::PlatformInternal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "PlatformInternal";
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
#[cfg(feature = "Oculus+Platform+PlatformInternal")]
impl std::ops::Deref for crate::Oculus::Platform::PlatformInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal")]
impl std::ops::DerefMut for crate::Oculus::Platform::PlatformInternal {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal")]
impl crate::Oculus::Platform::PlatformInternal {
    #[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
    pub type HTTP = crate::Oculus::Platform::PlatformInternal_HTTP;
    #[cfg(feature = "Oculus+Platform+PlatformInternal+MessageTypeInternal")]
    pub type MessageTypeInternal = crate::Oculus::Platform::PlatformInternal_MessageTypeInternal;
    #[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
    pub type Users = crate::Oculus::Platform::PlatformInternal_Users;
    pub fn CrashApplication() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CrashApplication")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CrashApplication", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeStandaloneAsync(
        appID: u64,
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::PlatformInitialize,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::PlatformInitialize,
                            >,
                        >,
                        2usize,
                    >("InitializeStandaloneAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeStandaloneAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::PlatformInitialize,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (appID, accessToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseMessageHandle(
        messageHandle: crate::System::IntPtr,
        messageType: crate::Oculus::Platform::Message_MessageType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            crate::Oculus::Platform::Message_MessageType,
                        ),
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
                        2usize,
                    >("ParseMessageHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseMessageHandle", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message> = unsafe {
            cordl_method_info.invoke_unchecked((), (messageHandle, messageType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::PlatformInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+HTTP")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformInternal_HTTP {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+HTTP")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::PlatformInternal_HTTP {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "PlatformInternal/HTTP";
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
#[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
impl std::ops::Deref for crate::Oculus::Platform::PlatformInternal_HTTP {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
impl std::ops::DerefMut for crate::Oculus::Platform::PlatformInternal_HTTP {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
impl crate::Oculus::Platform::PlatformInternal_HTTP {
    pub fn SetHttpTransferUpdateCallback(
        callback: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Oculus::Platform::Models::HttpTransferUpdate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::HttpTransferUpdate,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetHttpTransferUpdateCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetHttpTransferUpdateCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (callback))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+HTTP")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::PlatformInternal_HTTP {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+MessageTypeInternal")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlatformInternal_MessageTypeInternal {
    #[default]
    AbuseReport_LaunchAdvancedReportFlow = 1286683246u32,
    Application_ExecuteCoordinatedLaunch = 645772532u32,
    Application_GetInstalledApplications = 1376744524u32,
    Avatar_UpdateMetaData = 2077219214u32,
    Cal_FinalizeApplication = 497667029u32,
    Cal_GetSuggestedApplications = 1450209301u32,
    Cal_ProposeApplication = 1317270237u32,
    Colocation_GetCurrentMapUuid = 878018226u32,
    Colocation_RequestMap = 840263277u32,
    Colocation_ShareMap = 409847005u32,
    DeviceApplicationIntegrity_GetAttestationToken = 271557598u32,
    GraphAPI_Get = 822018158u32,
    GraphAPI_Post = 1990567876u32,
    HTTP_Get = 1874211363u32,
    HTTP_GetToFile = 1317133401u32,
    HTTP_MultiPartPost = 1480774160u32,
    HTTP_Post = 1798743375u32,
    Livestreaming_IsAllowedForApplication = 191729014u32,
    Livestreaming_StartPartyStream = 2066701532u32,
    Livestreaming_StartStream = 1343932350u32,
    Livestreaming_StopPartyStream = 661065560u32,
    Livestreaming_StopStream = 1155796426u32,
    Livestreaming_UpdateMicStatus = 475495815u32,
    NetSync_Connect = 1684899167u32,
    NetSync_Disconnect = 359268021u32,
    NetSync_GetSessions = 1859521077u32,
    NetSync_GetVoipAttenuation = 288016919u32,
    NetSync_GetVoipAttenuationDefault = 1467721888u32,
    NetSync_SetVoipAttenuation = 882366454u32,
    NetSync_SetVoipAttenuationModel = 1788128654u32,
    NetSync_SetVoipChannelCfg = 1553310963u32,
    NetSync_SetVoipGroup = 1477614734u32,
    NetSync_SetVoipListentoChannels = 1590749746u32,
    NetSync_SetVoipMicSource = 855832432u32,
    NetSync_SetVoipSessionMuted = 1434844938u32,
    NetSync_SetVoipSpeaktoChannels = 766496213u32,
    NetSync_SetVoipStreamMode = 1742839095u32,
    Party_Create = 450042703u32,
    Party_GatherInApplication = 1921499523u32,
    Party_Get = 1586058173u32,
    Party_GetCurrentForUser = 1489764138u32,
    Party_Invite = 901104867u32,
    Party_Join = 1744993395u32,
    Party_Leave = 848430801u32,
    RichPresence_SetDestination = 1328734477u32,
    RichPresence_SetIsJoinable = 1050353505u32,
    RichPresence_SetLobbySession = 1895893271u32,
    RichPresence_SetMatchSession = 1675623566u32,
    Room_CreateOrUpdateAndJoinNamed = 2089683601u32,
    Room_GetNamedRooms = 125660812u32,
    Room_GetSocialRooms = 1636310390u32,
    User_CancelRecordingForReportFlow = 65065289u32,
    User_GetLinkedAccounts = 1469314134u32,
    User_GetUserCapabilities = 303837564u32,
    User_LaunchReportFlow = 1449304081u32,
    User_LaunchReportFlow2 = 2139314275u32,
    User_NewEntitledTestUser = 292822787u32,
    User_NewTestUser = 921194380u32,
    User_NewTestUserFriends = 517416647u32,
    User_StartRecordingForReportFlow = 1819161571u32,
    User_StopRecordingAndLaunchReportFlow = 1618513035u32,
    User_StopRecordingAndLaunchReportFlow2 = 432190251u32,
    User_TestUserCreateDeviceManifest = 1701884605u32,
    Voip_ReportAppVoipSessions = 408048078u32,
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+MessageTypeInternal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::PlatformInternal_MessageTypeInternal {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "PlatformInternal/MessageTypeInternal";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+MessageTypeInternal")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Oculus::Platform::PlatformInternal_MessageTypeInternal {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+MessageTypeInternal")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Oculus::Platform::PlatformInternal_MessageTypeInternal {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+MessageTypeInternal")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Oculus::Platform::PlatformInternal_MessageTypeInternal {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+MessageTypeInternal")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Oculus::Platform::PlatformInternal_MessageTypeInternal {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+Users")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformInternal_Users {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+Users")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::PlatformInternal_Users {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "PlatformInternal/Users";
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
#[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
impl std::ops::Deref for crate::Oculus::Platform::PlatformInternal_Users {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
impl std::ops::DerefMut for crate::Oculus::Platform::PlatformInternal_Users {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
impl crate::Oculus::Platform::PlatformInternal_Users {
    pub fn GetLinkedAccounts(
        providers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::Oculus::Platform::ServiceProvider>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LinkedAccountList>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::Oculus::Platform::ServiceProvider,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::LinkedAccountList,
                            >,
                        >,
                        1usize,
                    >("GetLinkedAccounts")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLinkedAccounts", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LinkedAccountList>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (providers))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Oculus+Platform+PlatformInternal+Users")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::PlatformInternal_Users {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
