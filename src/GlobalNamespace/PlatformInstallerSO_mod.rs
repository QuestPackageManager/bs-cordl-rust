#[cfg(feature = "PlatformInstallerSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformInstallerSO {
    __cordl_parent: crate::Zenject::ScriptableObjectInstaller,
    pub _setupData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AppInitSetupData>,
    pub _ps4AchievementIdsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SonyAchievementIdsModelSO,
    >,
    pub _ps5AchievementIdsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SonyAchievementIdsModelSO,
    >,
    pub _achievementIdsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementIdsModelSO,
    >,
    pub _mockPlatformAdditionalContentModelInitialData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO,
    >,
}
#[cfg(feature = "PlatformInstallerSO")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PlatformInstallerSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformInstallerSO";
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
#[cfg(feature = "PlatformInstallerSO")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformInstallerSO {
    type Target = crate::Zenject::ScriptableObjectInstaller;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformInstallerSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformInstallerSO {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformInstallerSO")]
impl crate::GlobalNamespace::PlatformInstallerSO {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("InstallBindings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InstallBindings", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlatformInstallerSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlatformInstallerSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
