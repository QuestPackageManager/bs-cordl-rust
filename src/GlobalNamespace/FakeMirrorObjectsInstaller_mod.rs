#[cfg(feature = "FakeMirrorObjectsInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct FakeMirrorObjectsInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _mirroredGameNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredGameNoteController,
    >,
    pub _mirroredBurstSliderHeadGameNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredGameNoteController,
    >,
    pub _mirroredBurstSliderGameNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredGameNoteController,
    >,
    pub _mirroredBombNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredBombNoteController,
    >,
    pub _mirroredObstacleControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredObstacleController,
    >,
    pub _mirroredSliderControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirroredSliderController,
    >,
    pub _mirrorRendererGraphicsSettingsPresets: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets,
    >,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FakeMirrorObjectsInstaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FakeMirrorObjectsInstaller";
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
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::FakeMirrorObjectsInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::FakeMirrorObjectsInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl crate::GlobalNamespace::FakeMirrorObjectsInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::FakeMirrorObjectsInstaller as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InstallBindings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::FakeMirrorObjectsInstaller as
                    quest_hook::libil2cpp::Type > ::class(), "InstallBindings", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::FakeMirrorObjectsInstaller as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::FakeMirrorObjectsInstaller as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_mirroredGameNoteControllerPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MirroredGameNoteController>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::FakeMirrorObjectsInstaller as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MirroredGameNoteController,
                >,
                0usize,
            >("get_mirroredGameNoteControllerPrefab")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::FakeMirrorObjectsInstaller as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_mirroredGameNoteControllerPrefab", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MirroredGameNoteController,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FakeMirrorObjectsInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FakeMirrorObjectsInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
