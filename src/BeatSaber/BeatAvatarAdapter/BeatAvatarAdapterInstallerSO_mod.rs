#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarAdapterInstallerSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarAdapterInstallerSO {
    __cordl_parent: crate::BeatSaber::AvatarCore::AbstractAdapterInstallerSO,
    pub _avatarPartsModel: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO,
    pub _skinColorSet: *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSetSO,
    pub _settings: *mut crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystemSettings,
    pub _beatAvatarEditorViewController: *mut crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::BeatAvatarEditorViewController,
    pub _editAvatarColorView: *mut crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarColorViewController,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarAdapterInstallerSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarAdapter::BeatAvatarAdapterInstallerSO =>
    "BeatSaber.BeatAvatarAdapter"."BeatAvatarAdapterInstallerSO"
);
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstallBindings_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstallBindings_DiContainer1(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", (container))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
