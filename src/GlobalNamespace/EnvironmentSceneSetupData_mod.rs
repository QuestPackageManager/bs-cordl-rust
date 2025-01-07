#[cfg(feature = "EnvironmentSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub hideBranding: bool,
    pub environmentInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentInfoSO,
    >,
    pub beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
}
#[cfg(feature = "EnvironmentSceneSetupData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EnvironmentSceneSetupData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EnvironmentSceneSetupData";
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
#[cfg(feature = "EnvironmentSceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentSceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentSceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentSceneSetupData")]
impl crate::GlobalNamespace::EnvironmentSceneSetupData {
    pub fn New(
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        hideBranding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (environmentInfo, beatmapLevel, hideBranding))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        hideBranding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (environmentInfo, beatmapLevel, hideBranding))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EnvironmentSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
