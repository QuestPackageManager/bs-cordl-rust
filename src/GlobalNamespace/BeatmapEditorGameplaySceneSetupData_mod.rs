#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEditorGameplaySceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub data: crate::GlobalNamespace::BeatmapEditorStartTestLevelData,
    pub recordVRMovement: bool,
    pub playVRMovement: bool,
}
#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapEditorGameplaySceneSetupData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEditorGameplaySceneSetupData";
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
#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEditorGameplaySceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapEditorGameplaySceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
impl crate::GlobalNamespace::BeatmapEditorGameplaySceneSetupData {
    pub fn New(
        recordVRMovement: bool,
        playVRMovement: bool,
        data: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapEditorStartTestLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (recordVRMovement, playVRMovement, data))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        recordVRMovement: bool,
        playVRMovement: bool,
        data: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapEditorStartTestLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            bool,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::BeatmapEditorStartTestLevelData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (recordVRMovement, playVRMovement, data))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapEditorGameplaySceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapEditorGameplaySceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
