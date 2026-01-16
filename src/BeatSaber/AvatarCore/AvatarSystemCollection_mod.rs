#[cfg(feature = "cordl_class_BeatSaber+AvatarCore+AvatarSystemCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarSystemCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _availableAvatarSystems: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u32,
            quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
        >,
    >,
    pub _availableUserSelectableAvatarSystems: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
        >,
    >,
    pub _supportedOptionalAvatarDataTypes:
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IReadOnlyCollection_1<u32>>,
    pub _fallbackAvatarSystem:
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    pub _availableAvatarSystemList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
        >,
    >,
}
#[cfg(feature = "cordl_class_BeatSaber+AvatarCore+AvatarSystemCollection")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::AvatarCore::AvatarSystemCollection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "AvatarSystemCollection";
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
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::AvatarSystemCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::AvatarSystemCollection {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemCollection")]
impl crate::BeatSaber::AvatarCore::AvatarSystemCollection {
    pub fn AvatarSystemBySelectionSortOrder(
        system1: quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
        system2: quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
                        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
                    ), i32, 2usize>("AvatarSystemBySelectionSortOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AvatarSystemBySelectionSortOrder",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (system1, system2))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDefaultAvatarsForAvatarSystemsWithoutUserCreatedAvatarAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        0usize,
                    >(
                        "CreateDefaultAvatarsForAvatarSystemsWithoutUserCreatedAvatarAsync",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDefaultAvatarsForAvatarSystemsWithoutUserCreatedAvatarAsync",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAvatarSystem_AvatarSystemIdentifier1(
        &mut self,
        avatarSystemIdentifier: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::AvatarCore::AvatarSystemIdentifier),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::AvatarCore::IAvatarSystem,
                        >,
                        1usize,
                    >("GetAvatarSystem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAvatarSystem", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem> =
            unsafe { cordl_method_info.invoke_unchecked(self, (avatarSystemIdentifier))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAvatarSystem_IAvatarSystemMetadata0(
        &mut self,
        avatarSystemMetadata: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::AvatarCore::IAvatarSystem,
                        >,
                        1usize,
                    >("GetAvatarSystem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAvatarSystem", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem> =
            unsafe { cordl_method_info.invoke_unchecked(self, (avatarSystemMetadata))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAvatarSystem_Il2CppString2(
        &mut self,
        avatarTypeIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::AvatarCore::IAvatarSystem,
                        >,
                        1usize,
                    >("GetAvatarSystem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAvatarSystem", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem> =
            unsafe { cordl_method_info.invoke_unchecked(self, (avatarTypeIdentifier))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAvatarSystem_u32_3(
        &mut self,
        avatarTypeIdentifierHash: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::AvatarCore::IAvatarSystem,
                        >,
                        1usize,
                    >("GetAvatarSystem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAvatarSystem", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem> =
            unsafe { cordl_method_info.invoke_unchecked(self, (avatarTypeIdentifierHash))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMultiplayerAvatarOptionalDataProvider(
        &mut self,
        selectedAvatarTypeIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
                        >,
                        1usize,
                    >("GetMultiplayerAvatarOptionalDataProvider")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMultiplayerAvatarOptionalDataProvider", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (selectedAvatarTypeIdentifier))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMultiplayerAvatarsData(
        &mut self,
        selectedAvatarTypeIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::GlobalNamespace::MultiplayerAvatarsData>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::GlobalNamespace::MultiplayerAvatarsData,
                            >,
                        >,
                        1usize,
                    >("GetMultiplayerAvatarsData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMultiplayerAvatarsData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::GlobalNamespace::MultiplayerAvatarsData>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (selectedAvatarTypeIdentifier))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasAvatarSystem(
        &mut self,
        avatarTypeIdentifierHash: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), bool, 1usize>("HasAvatarSystem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HasAvatarSystem",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (avatarTypeIdentifierHash))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        boundAvatarSystems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (boundAvatarSystems))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        boundAvatarSystems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystem>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (boundAvatarSystems))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_anyAvatarCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<bool>,
                        >,
                        0usize,
                    >("get_anyAvatarCreated")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_anyAvatarCreated", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_availableAvatarSystems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystemMetadata>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IReadOnlyList_1<
                            quest_hook::libil2cpp::Gc<
                                crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
                            >,
                        >,
                    >, 0usize>("get_availableAvatarSystems")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_availableAvatarSystems",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystemMetadata>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_selectableAvatarSystems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystemMetadata>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IReadOnlyList_1<
                            quest_hook::libil2cpp::Gc<
                                crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
                            >,
                        >,
                    >, 0usize>("get_selectableAvatarSystems")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_selectableAvatarSystems",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::IAvatarSystemMetadata>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_supportedOptionalAvatarDataType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IReadOnlyCollection_1<u32>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IReadOnlyCollection_1<u32>,
                    >, 0usize>("get_supportedOptionalAvatarDataType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_supportedOptionalAvatarDataType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyCollection_1<u32>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+AvatarCore+AvatarSystemCollection")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::AvatarCore::AvatarSystemCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
