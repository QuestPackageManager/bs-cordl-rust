#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Colocation+INetworkData")]
#[derive(Debug)]
#[repr(C)]
pub struct INetworkData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Colocation+INetworkData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.MultiplayerBlocks.Colocation";
    const CLASS_NAME: &'static str = "INetworkData";
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
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Colocation+INetworkData")]
impl std::ops::Deref for crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Colocation+INetworkData")]
impl std::ops::DerefMut for crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Colocation+INetworkData")]
impl crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData {
    pub fn AddAnchor(
        &mut self,
        anchor: crate::Meta::XR::MultiplayerBlocks::Colocation::Anchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Meta::XR::MultiplayerBlocks::Colocation::Anchor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddAnchor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddAnchor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (anchor))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddPlayer(
        &mut self,
        player: crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Meta::XR::MultiplayerBlocks::Colocation::Player),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddPlayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddPlayer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (player))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Meta::XR::MultiplayerBlocks::Colocation::Anchor,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::Meta::XR::MultiplayerBlocks::Colocation::Anchor,
                        >,
                    >, 0usize>("GetAllAnchors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllAnchors",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Meta::XR::MultiplayerBlocks::Colocation::Anchor,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
                        >,
                    >, 0usize>("GetAllPlayers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllPlayers",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAnchor(
        &mut self,
        ownerOculusId: u64,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Meta::XR::MultiplayerBlocks::Colocation::Anchor>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), crate::System::Nullable_1<
                        crate::Meta::XR::MultiplayerBlocks::Colocation::Anchor,
                    >, 1usize>("GetAnchor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAnchor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::Meta::XR::MultiplayerBlocks::Colocation::Anchor,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ownerOculusId))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetColocationGroupCount(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("GetColocationGroupCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetColocationGroupCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerWithOculusId(
        &mut self,
        oculusId: u64,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Meta::XR::MultiplayerBlocks::Colocation::Player>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), crate::System::Nullable_1<
                        crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
                    >, 1usize>("GetPlayerWithOculusId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPlayerWithOculusId",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (oculusId))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerWithPlayerId(
        &mut self,
        playerId: u64,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Meta::XR::MultiplayerBlocks::Colocation::Player>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), crate::System::Nullable_1<
                        crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
                    >, 1usize>("GetPlayerWithPlayerId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPlayerWithPlayerId",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (playerId))? };
        Ok(__cordl_ret.into())
    }
    pub fn IncrementColocationGroupCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "IncrementColocationGroupCount",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IncrementColocationGroupCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAnchor(
        &mut self,
        anchor: crate::Meta::XR::MultiplayerBlocks::Colocation::Anchor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Meta::XR::MultiplayerBlocks::Colocation::Anchor),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RemoveAnchor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveAnchor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (anchor))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemovePlayer(
        &mut self,
        player: crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Meta::XR::MultiplayerBlocks::Colocation::Player),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RemovePlayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemovePlayer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (player))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Colocation+INetworkData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::MultiplayerBlocks::Colocation::INetworkData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
