#[cfg(feature = "cordl_class_LocalServerNetworkPlayerModel")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalServerNetworkPlayerModel {
    __cordl_parent: crate::GlobalNamespace::NetworkPlayerModel_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalServerConnectionManager>,
    >,
    pub _cachedConnectToServerParams: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LocalServerConnectionManager_LocalServerConnectionManagerConnectToServerParams,
    >,
    pub _cachedStartClientParams: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LocalServerConnectionManager_LocalServerConnectionManagerStartClientParams,
    >,
    pub _localPort: i32,
}
#[cfg(feature = "cordl_class_LocalServerNetworkPlayerModel")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::LocalServerNetworkPlayerModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LocalServerNetworkPlayerModel";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "LocalServerNetworkPlayerModel")]
impl std::ops::Deref for crate::GlobalNamespace::LocalServerNetworkPlayerModel {
    type Target = crate::GlobalNamespace::NetworkPlayerModel_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalServerConnectionManager>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalServerNetworkPlayerModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalServerNetworkPlayerModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalServerNetworkPlayerModel")]
impl crate::GlobalNamespace::LocalServerNetworkPlayerModel {
    #[cfg(feature = "LocalServerNetworkPlayerModel+LocalServerJoinMatchmakingPartyConfig")]
    pub type LocalServerJoinMatchmakingPartyConfig =
        crate::GlobalNamespace::LocalServerNetworkPlayerModel_LocalServerJoinMatchmakingPartyConfig;
    pub fn CreatePartyConnection<T2>(
        &mut self,
        config: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::INetworkPlayerModelPartyConfig_1<T2>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::INetworkPlayerModelPartyConfig_1<T2>,
                    >), bool, 1usize>("CreatePartyConnection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreatePartyConnection",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (config))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetConnectToServerParams(
        &mut self,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        configuration: crate::GlobalNamespace::GameplayServerConfiguration,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionInitParams_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalServerConnectionManager>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::BeatmapLevelSelectionMask,
                        crate::GlobalNamespace::GameplayServerConfiguration,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IConnectionInitParams_1<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LocalServerConnectionManager,
                            >,
                        >,
                    >, 4usize>("GetConnectToServerParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetConnectToServerParams",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionInitParams_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalServerConnectionManager>,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (selectionMask, configuration, secret, code))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStartClientParams(
        &mut self,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        configuration: crate::GlobalNamespace::GameplayServerConfiguration,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionInitParams_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalServerConnectionManager>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::BeatmapLevelSelectionMask,
                        crate::GlobalNamespace::GameplayServerConfiguration,
                    ), quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IConnectionInitParams_1<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LocalServerConnectionManager,
                            >,
                        >,
                    >, 2usize>("GetStartClientParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetStartClientParams",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionInitParams_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalServerConnectionManager>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (selectionMask, configuration))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshPublicServers(
        &mut self,
        localSelectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        localConfiguration: crate::GlobalNamespace::GameplayServerConfiguration,
        onSuccess: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        crate::GlobalNamespace::PublicServerInfo,
                    >,
                >,
            >,
        >,
        onFailure: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::ConnectionFailedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::BeatmapLevelSelectionMask,
                        crate::GlobalNamespace::GameplayServerConfiguration,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::IReadOnlyList_1<
                                        crate::GlobalNamespace::PublicServerInfo,
                                    >,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<crate::GlobalNamespace::ConnectionFailedReason>,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "RefreshPublicServers"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RefreshPublicServers",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (localSelectionMask, localConfiguration, onSuccess, onFailure),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_code(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_code")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_code", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_configuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::GameplayServerConfiguration> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::GameplayServerConfiguration, 0usize>(
                        "get_configuration",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_configuration",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::GameplayServerConfiguration =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_partyOwnerId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_partyOwnerId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_partyOwnerId", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_secret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_secret")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_secret", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_selectionMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapLevelSelectionMask> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::BeatmapLevelSelectionMask, 0usize>(
                        "get_selectionMask",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_selectionMask",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelSelectionMask =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_LocalServerNetworkPlayerModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LocalServerNetworkPlayerModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_LocalServerNetworkPlayerModel+LocalServerJoinMatchmakingPartyConfig")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalServerNetworkPlayerModel_LocalServerJoinMatchmakingPartyConfig {
    __cordl_parent: crate::GlobalNamespace::NetworkPlayerModel_1_JoinMatchmakingPartyConfig<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalServerConnectionManager>,
    >,
    pub localPort: i32,
}
#[cfg(feature = "cordl_class_LocalServerNetworkPlayerModel+LocalServerJoinMatchmakingPartyConfig")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::LocalServerNetworkPlayerModel_LocalServerJoinMatchmakingPartyConfig
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str =
        "LocalServerNetworkPlayerModel/LocalServerJoinMatchmakingPartyConfig";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "LocalServerNetworkPlayerModel+LocalServerJoinMatchmakingPartyConfig")]
impl std::ops::Deref
    for crate::GlobalNamespace::LocalServerNetworkPlayerModel_LocalServerJoinMatchmakingPartyConfig
{
    type Target = crate::GlobalNamespace::NetworkPlayerModel_1_JoinMatchmakingPartyConfig<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalServerConnectionManager>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalServerNetworkPlayerModel+LocalServerJoinMatchmakingPartyConfig")]
impl std::ops::DerefMut
    for crate::GlobalNamespace::LocalServerNetworkPlayerModel_LocalServerJoinMatchmakingPartyConfig
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalServerNetworkPlayerModel+LocalServerJoinMatchmakingPartyConfig")]
impl crate::GlobalNamespace::LocalServerNetworkPlayerModel_LocalServerJoinMatchmakingPartyConfig {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_LocalServerNetworkPlayerModel+LocalServerJoinMatchmakingPartyConfig")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::LocalServerNetworkPlayerModel_LocalServerJoinMatchmakingPartyConfig
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
