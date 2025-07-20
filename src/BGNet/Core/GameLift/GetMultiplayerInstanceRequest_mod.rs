#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetMultiplayerInstanceRequest {
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub serviceEnvironment: crate::GlobalNamespace::ServiceEnvironment,
    pub singleUseAuthToken: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub beatmapLevelSelectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    pub gameplayServerConfiguration: crate::GlobalNamespace::GameplayServerConfiguration,
    pub userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub privateGameSecret: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub privateGameCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub platform: crate::GlobalNamespace::AuthenticationToken_Platform,
    pub authUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub gameliftRegionLatencies: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            i64,
        >,
    >,
    pub ticketId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub placementId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceRequest {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BGNet.Core.GameLift";
    const CLASS_NAME: &'static str = "GetMultiplayerInstanceRequest";
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
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceRequest {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceRequest {
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
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceRequest {
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
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceRequest {
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
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceRequest {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceRequest")]
impl crate::BGNet::Core::GameLift::GetMultiplayerInstanceRequest {
    pub fn _ctor(
        &mut self,
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serviceEnvironment: crate::GlobalNamespace::ServiceEnvironment,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevelSelectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        gameplayServerConfiguration: crate::GlobalNamespace::GameplayServerConfiguration,
        platform: crate::GlobalNamespace::AuthenticationToken_Platform,
        authUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        singleUseAuthToken: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        privateGameSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        privateGameCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameliftRegionLatencies: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                i64,
            >,
        >,
        ticketId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        placementId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::ServiceEnvironment,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::BeatmapLevelSelectionMask,
                            crate::GlobalNamespace::GameplayServerConfiguration,
                            crate::GlobalNamespace::AuthenticationToken_Platform,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                    i64,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        13usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            13usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        version,
                        serviceEnvironment,
                        userId,
                        beatmapLevelSelectionMask,
                        gameplayServerConfiguration,
                        platform,
                        authUserId,
                        singleUseAuthToken,
                        privateGameSecret,
                        privateGameCode,
                        gameliftRegionLatencies,
                        ticketId,
                        placementId,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
