#[cfg(feature = "SettingsIO")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingsIO {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "SettingsIO")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SettingsIO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SettingsIO";
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
#[cfg(feature = "SettingsIO")]
impl std::ops::Deref for crate::GlobalNamespace::SettingsIO {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsIO")]
impl std::ops::DerefMut for crate::GlobalNamespace::SettingsIO {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SettingsIO")]
impl crate::GlobalNamespace::SettingsIO {
    pub const kGraphicsSettingsPath: &'static str = "GraphicsSettings.json";
    pub const kLegacySettingsPath: &'static str = "settings.cfg";
    pub const kMainSettingsPath: &'static str = "MainSettings.json";
    pub const kSettingsPath: &'static str = "settings.ini";
    pub fn Decode(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::BeatSaber::Settings::Settings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        2usize,
                    >("Decode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Decode", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (settings, text))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::BeatSaber::Settings::Settings,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("Encode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Encode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (settings))? };
        Ok(__cordl_ret.into())
    }
    pub fn Load(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        platform: crate::GlobalNamespace::HardwareCategory,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Settings::Settings> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IFileStorage,
                            >,
                            crate::GlobalNamespace::HardwareCategory,
                        ),
                        crate::BeatSaber::Settings::Settings,
                        2usize,
                    >("Load")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Load", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Settings::Settings = unsafe {
            method.invoke_unchecked((), (fileStorage, platform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsync(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        platform: crate::GlobalNamespace::HardwareCategory,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::BeatSaber::Settings::Settings>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IFileStorage,
                            >,
                            crate::GlobalNamespace::HardwareCategory,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::BeatSaber::Settings::Settings,
                            >,
                        >,
                        2usize,
                    >("LoadAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::BeatSaber::Settings::Settings>,
        > = unsafe { method.invoke_unchecked((), (fileStorage, platform))? };
        Ok(__cordl_ret.into())
    }
    pub fn SaveAsync(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        settings: crate::BeatSaber::Settings::Settings,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IFileStorage,
                            >,
                            crate::BeatSaber::Settings::Settings,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        2usize,
                    >("SaveAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SaveAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked((), (fileStorage, settings))? };
        Ok(__cordl_ret.into())
    }
    pub fn WipeAsync(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IFileStorage,
                        >),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        1usize,
                    >("WipeAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WipeAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked((), (fileStorage))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SettingsIO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SettingsIO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
