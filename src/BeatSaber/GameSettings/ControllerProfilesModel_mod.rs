#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllerProfilesModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _fileModel: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::GameSettings::ControllerProfileFileModel,
    >,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _selectedProfileIndex_k__BackingField: i32,
    pub _profiles: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::GameSettings::ControllerProfile>,
        >,
    >,
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfilesModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::GameSettings::ControllerProfilesModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.GameSettings";
    const CLASS_NAME: &'static str = "ControllerProfilesModel";
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
    pub fn GetBuiltInProfiles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::GameSettings::ControllerProfile,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::GameSettings::ControllerProfile,
                        >,
                    >,
                >,
                0usize,
            >("GetBuiltInProfiles")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), "GetBuiltInProfiles", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::GameSettings::ControllerProfile,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerProfilesSaveData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::GameSettings::ControllerProfileSaveData,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::GameSettings::ControllerProfileSaveData,
                        >,
                    >,
                >,
                0usize,
            >("GetControllerProfilesSaveData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetControllerProfilesSaveData", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::GameSettings::ControllerProfileSaveData,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultController() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::GameSettings::Controller,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::BeatSaber::GameSettings::Controller,
                0usize,
            >("GetDefaultController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), "GetDefaultController",
                    0usize
                )
            });
        let __cordl_ret: crate::BeatSaber::GameSettings::Controller = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultControllersProfile(
        localizationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        modifiable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::GameSettings::ControllerProfile>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::GameSettings::ControllerProfile,
                >,
                3usize,
            >("GetDefaultControllersProfile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetDefaultControllersProfile", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfile,
        > = unsafe {
            method.invoke_unchecked((), (localizationKey, index, modifiable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultCustomControllerProfile(
        &mut self,
        profileIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::GameSettings::ControllerProfile>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::GameSettings::ControllerProfile,
                >,
                1usize,
            >("GetDefaultCustomControllerProfile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetDefaultCustomControllerProfile", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfile,
        > = unsafe { method.invoke_unchecked(self, (profileIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSelectedProfileIndexFromSettings(
        &mut self,
        builtInProfilesCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetSelectedProfileIndexFromSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetSelectedProfileIndexFromSettings", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (builtInProfilesCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), "Initialize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                0usize,
            >("LoadAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), "LoadAsync", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RefreshControllersReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "RefreshControllersReference", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SaveAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                0usize,
            >("SaveAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), "SaveAsync", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSelectedProfile(
        &mut self,
        newSelectedProfileIndex: i32,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UpdateSelectedProfile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateSelectedProfile",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (newSelectedProfileIndex, forceUpdate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_profiles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::GameSettings::ControllerProfile,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::GameSettings::ControllerProfile,
                        >,
                    >,
                >,
                0usize,
            >("get_profiles")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), "get_profiles", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::GameSettings::ControllerProfile,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedProfile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::GameSettings::ControllerProfile>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::GameSettings::ControllerProfile,
                >,
                0usize,
            >("get_selectedProfile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), "get_selectedProfile",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfile,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedProfileIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_selectedProfileIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), "get_selectedProfileIndex",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_selectedProfileIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::GameSettings::ControllerProfilesModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_selectedProfileIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::GameSettings::ControllerProfilesModel as
                    quest_hook::libil2cpp::Type > ::class(), "set_selectedProfileIndex",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
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
