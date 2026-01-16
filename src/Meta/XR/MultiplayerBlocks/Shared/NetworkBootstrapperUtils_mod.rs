#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Shared+NetworkBootstrapperUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkBootstrapperUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Shared+NetworkBootstrapperUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Meta::XR::MultiplayerBlocks::Shared::NetworkBootstrapperUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.MultiplayerBlocks.Shared";
    const CLASS_NAME: &'static str = "NetworkBootstrapperUtils";
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
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Shared+NetworkBootstrapperUtils")]
impl std::ops::Deref
for crate::Meta::XR::MultiplayerBlocks::Shared::NetworkBootstrapperUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Shared+NetworkBootstrapperUtils")]
impl std::ops::DerefMut
for crate::Meta::XR::MultiplayerBlocks::Shared::NetworkBootstrapperUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Shared+NetworkBootstrapperUtils")]
impl crate::Meta::XR::MultiplayerBlocks::Shared::NetworkBootstrapperUtils {
    pub fn OnColocationFailed(
        e: crate::Meta::XR::MultiplayerBlocks::Colocation::ColocationFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Meta::XR::MultiplayerBlocks::Colocation::ColocationFailedReason),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnColocationFailed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnColocationFailed", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetEntitlementIds(
        info: crate::Meta::XR::MultiplayerBlocks::Shared::PlatformInfo,
        param: quest_hook::libil2cpp::ByRefMut<
            crate::Meta::XR::MultiplayerBlocks::Shared::NetworkBootstrapperParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Meta::XR::MultiplayerBlocks::Shared::PlatformInfo,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Meta::XR::MultiplayerBlocks::Shared::NetworkBootstrapperParams,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetEntitlementIds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetEntitlementIds", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (info, param))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetUpAndStartAutomaticColocation(
        param: quest_hook::libil2cpp::ByRefMut<
            crate::Meta::XR::MultiplayerBlocks::Shared::NetworkBootstrapperParams,
        >,
        anchorPrefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        networkData: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData,
        >,
        networkMessenger: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkMessenger,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Meta::XR::MultiplayerBlocks::Shared::NetworkBootstrapperParams,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkMessenger,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetUpAndStartAutomaticColocation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetUpAndStartAutomaticColocation", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (param, anchorPrefab, networkData, networkMessenger),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Shared+NetworkBootstrapperUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::MultiplayerBlocks::Shared::NetworkBootstrapperUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
