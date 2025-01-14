#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarAdapterInstallerSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarAdapterInstallerSO {
    __cordl_parent: crate::BeatSaber::AvatarCore::AbstractAdapterInstallerSO,
    pub _avatarPartsModel: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO,
    >,
    pub _skinColorSet: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::SkinColorSetSO,
    >,
    pub _settings: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystemSettings,
    >,
    pub _beatAvatarEditorViewController: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::BeatAvatarEditorViewController,
    >,
    pub _editAvatarColorView: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarColorViewController,
    >,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarAdapterInstallerSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::BeatAvatarAdapter::BeatAvatarAdapterInstallerSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarAdapter";
    const CLASS_NAME: &'static str = "BeatAvatarAdapterInstallerSO";
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
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarAdapterInstallerSO")]
impl std::ops::Deref
for crate::BeatSaber::BeatAvatarAdapter::BeatAvatarAdapterInstallerSO {
    type Target = crate::BeatSaber::AvatarCore::AbstractAdapterInstallerSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarAdapterInstallerSO")]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarAdapter::BeatAvatarAdapterInstallerSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarAdapterInstallerSO")]
impl crate::BeatSaber::BeatAvatarAdapter::BeatAvatarAdapterInstallerSO {
    pub fn InstallBindings_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InstallBindings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InstallBindings", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn InstallBindings_DiContainer1(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InstallBindings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InstallBindings", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container))
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarAdapterInstallerSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarAdapter::BeatAvatarAdapterInstallerSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
