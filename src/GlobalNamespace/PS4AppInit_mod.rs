#[cfg(feature = "PS4AppInit")]
#[repr(C)]
#[derive(Debug)]
pub struct PS4AppInit {
    __cordl_parent: crate::BeatSaber::Init::BSAppInit,
    pub _mainSystemInit: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainSystemInit,
    >,
    pub _settingsApplicator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsApplicatorSO,
    >,
    pub _defaultScenesTransitionsFromInit: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DefaultScenesTransitionsFromInit,
    >,
    pub _appInitScenesTransitionSetupDataContainer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AppInitScenesTransitionSetupDataContainerSO,
    >,
    pub _activePublisherSKUSettingsSO: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PS4ActivePublisherSKUSettingsSO,
    >,
    pub _sonyOnGoingToBackgroundSaveHandler: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SonyOnGoingToBackgroundSaveHandler,
    >,
}
#[cfg(feature = "PS4AppInit")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PS4AppInit {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PS4AppInit";
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
#[cfg(feature = "PS4AppInit")]
impl std::ops::Deref for crate::GlobalNamespace::PS4AppInit {
    type Target = crate::BeatSaber::Init::BSAppInit;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS4AppInit")]
impl std::ops::DerefMut for crate::GlobalNamespace::PS4AppInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS4AppInit")]
impl crate::GlobalNamespace::PS4AppInit {
    pub fn AppStartAndMultiSceneEditorSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppStartAndMultiSceneEditorSetup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeModules() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeModules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PreloadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("PreloadAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RepeatableSetupAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("RepeatableSetupAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TransitionToNextScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionToNextScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __n__0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("<>n__0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PS4AppInit")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PS4AppInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
