#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Colocation+NetworkDataUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkDataUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Colocation+NetworkDataUtils")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::MultiplayerBlocks::Colocation::NetworkDataUtils
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.MultiplayerBlocks.Colocation";
    const CLASS_NAME: &'static str = "NetworkDataUtils";
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
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Colocation+NetworkDataUtils")]
impl std::ops::Deref for crate::Meta::XR::MultiplayerBlocks::Colocation::NetworkDataUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Colocation+NetworkDataUtils")]
impl std::ops::DerefMut for crate::Meta::XR::MultiplayerBlocks::Colocation::NetworkDataUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+MultiplayerBlocks+Colocation+NetworkDataUtils")]
impl crate::Meta::XR::MultiplayerBlocks::Colocation::NetworkDataUtils {
    pub fn GetAllPlayersColocatedWith(
        oculusId: u64,
        includeMyself: bool,
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
                    .find_static_method::<(u64, bool), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
                        >,
                    >, 2usize>("GetAllPlayersColocatedWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllPlayersColocatedWith",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (oculusId, includeMyself))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllPlayersFromColocationGroupId(
        colocationGroupId: u32,
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
                    .find_static_method::<(u32), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
                        >,
                    >, 1usize>("GetAllPlayersFromColocationGroupId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllPlayersFromColocationGroupId",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (colocationGroupId))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetOculusIdOfColocatedGroupOwnerFromColocationGroupId(
        colocationGroupId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<u64>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32), crate::System::Nullable_1<u64>, 1usize>(
                        "GetOculusIdOfColocatedGroupOwnerFromColocationGroupId",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetOculusIdOfColocatedGroupOwnerFromColocationGroupId",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<u64> =
            unsafe { cordl_method_info.invoke_unchecked((), (colocationGroupId))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerFromOculusId(
        oculusId: u64,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Meta::XR::MultiplayerBlocks::Colocation::Player>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64), crate::System::Nullable_1<
                        crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
                    >, 1usize>("GetPlayerFromOculusId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPlayerFromOculusId",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::Meta::XR::MultiplayerBlocks::Colocation::Player,
        > = unsafe { cordl_method_info.invoke_unchecked((), (oculusId))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+MultiplayerBlocks+Colocation+NetworkDataUtils")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::MultiplayerBlocks::Colocation::NetworkDataUtils
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
