#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetMultiplayerInstanceResponse {
    pub errorCode: crate::GlobalNamespace::MultiplayerPlacementErrorCode,
    pub playerSessionInfo: quest_hook::libil2cpp::Gc<
        crate::BGNet::Core::GameLift::PlayerSessionInfo,
    >,
    pub pollIntervalMs: i32,
    pub ticketId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ticketStatus: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub placementId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub placementStatus: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceResponse {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BGNet.Core.GameLift";
    const CLASS_NAME: &'static str = "GetMultiplayerInstanceResponse";
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
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceResponse {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceResponse {
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
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceResponse {
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
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceResponse {
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
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BGNet::Core::GameLift::GetMultiplayerInstanceResponse {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+GetMultiplayerInstanceResponse")]
impl crate::BGNet::Core::GameLift::GetMultiplayerInstanceResponse {
    pub fn _ctor(
        &mut self,
        errorCode: crate::GlobalNamespace::MultiplayerPlacementErrorCode,
        playerSessionInfo: quest_hook::libil2cpp::Gc<
            crate::BGNet::Core::GameLift::PlayerSessionInfo,
        >,
        pollIntervalMs: i32,
        ticketId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ticketStatus: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        placementId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        placementStatus: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::MultiplayerPlacementErrorCode,
                            quest_hook::libil2cpp::Gc<
                                crate::BGNet::Core::GameLift::PlayerSessionInfo,
                            >,
                            i32,
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
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        errorCode,
                        playerSessionInfo,
                        pollIntervalMs,
                        ticketId,
                        ticketStatus,
                        placementId,
                        placementStatus,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
