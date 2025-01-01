#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllerProfilesModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _fileModel: *mut crate::BeatSaber::GameSettings::ControllerProfileFileModel,
    pub _vrPlatformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
    pub _settingsManager: *mut crate::GlobalNamespace::SettingsManager,
    pub _selectedProfileIndex_k__BackingField: i32,
    pub _profiles: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatSaber::GameSettings::ControllerProfile,
    >,
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::ControllerProfilesModel
    => "BeatSaber.GameSettings"."ControllerProfilesModel"
);
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::ControllerProfilesModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::ControllerProfilesModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel")]
impl crate::BeatSaber::GameSettings::ControllerProfilesModel {
    pub const kCustomLocalizationKey: &'static str = "CONTROLLER_PROFILES_LABEL_CUSTOM";
    pub const kCustomProfilesCount: i32 = 5i32;
    pub const kDefaultLocalizationKey: &'static str = "CONTROLLER_PROFILES_LABEL_DEFAULT";
    #[cfg(
        feature = "BeatSaber+GameSettings+ControllerProfilesModel+_GetBuiltInProfiles_d__21"
    )]
    pub type _GetBuiltInProfiles_d__21 = crate::BeatSaber::GameSettings::ControllerProfilesModel__GetBuiltInProfiles_d__21;
    #[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel+_Initialize_d__16")]
    pub type _Initialize_d__16 = crate::BeatSaber::GameSettings::ControllerProfilesModel__Initialize_d__16;
    #[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel+_LoadAsync_d__17")]
    pub type _LoadAsync_d__17 = crate::BeatSaber::GameSettings::ControllerProfilesModel__LoadAsync_d__17;
    #[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel+__c")]
    pub type __c = crate::BeatSaber::GameSettings::ControllerProfilesModel___c;
    pub fn GetBuiltInProfiles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::BeatSaber::GameSettings::ControllerProfile,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::BeatSaber::GameSettings::ControllerProfile,
            >,
        > = __cordl_object.invoke("GetBuiltInProfiles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerProfilesSaveData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::BeatSaber::GameSettings::ControllerProfileSaveData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::BeatSaber::GameSettings::ControllerProfileSaveData,
            >,
        > = __cordl_object.invoke("GetControllerProfilesSaveData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultCustomControllerProfile(
        &mut self,
        profileIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::GameSettings::ControllerProfile>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfile,
        > = __cordl_object.invoke("GetDefaultCustomControllerProfile", (profileIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedProfileIndexFromSettings(
        &mut self,
        builtInProfilesCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetSelectedProfileIndexFromSettings", (builtInProfilesCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("LoadAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshControllersReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshControllersReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("SaveAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSelectedProfile(
        &mut self,
        newSelectedProfileIndex: i32,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSelectedProfile", (newSelectedProfileIndex, forceUpdate))?;
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
    pub fn get_profiles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatSaber::GameSettings::ControllerProfile,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatSaber::GameSettings::ControllerProfile,
            >,
        > = __cordl_object.invoke("get_profiles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedProfile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::GameSettings::ControllerProfile>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfile,
        > = __cordl_object.invoke("get_selectedProfile", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedProfileIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectedProfileIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectedProfileIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectedProfileIndex", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::ControllerProfilesModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel")]
impl AsRef<crate::Zenject::IInitializable>
for crate::BeatSaber::GameSettings::ControllerProfilesModel {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel")]
impl AsMut<crate::Zenject::IInitializable>
for crate::BeatSaber::GameSettings::ControllerProfilesModel {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
