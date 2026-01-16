#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Colocation+NetworkAdapter")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkAdapter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Colocation+NetworkAdapter")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::MultiplayerBlocks::Colocation::NetworkAdapter
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.MultiplayerBlocks.Colocation";
    const CLASS_NAME: &'static str = "NetworkAdapter";
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
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Colocation+NetworkAdapter")]
impl std::ops::Deref for crate::Meta::XR::MultiplayerBlocks::Colocation::NetworkAdapter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Colocation+NetworkAdapter")]
impl std::ops::DerefMut for crate::Meta::XR::MultiplayerBlocks::Colocation::NetworkAdapter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Colocation+NetworkAdapter")]
impl crate::Meta::XR::MultiplayerBlocks::Colocation::NetworkAdapter {
    pub fn SetConfig(
        networkData: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData,
        >,
        networkMessenger: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkMessenger,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkMessenger,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetConfig")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetConfig",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (networkData, networkMessenger))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NetworkData() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData,
                    >, 0usize>("get_NetworkData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_NetworkData",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NetworkMessenger() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkMessenger,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkMessenger,
                    >, 0usize>("get_NetworkMessenger")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_NetworkMessenger",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkMessenger,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_NetworkData(
        value: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData,
                    >), quest_hook::libil2cpp::Void, 1usize>("set_NetworkData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_NetworkData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_NetworkMessenger(
        value: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkMessenger,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkMessenger,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "set_NetworkMessenger"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_NetworkMessenger",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Colocation+NetworkAdapter")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::MultiplayerBlocks::Colocation::NetworkAdapter
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
