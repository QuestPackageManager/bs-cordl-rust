#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::GraphQLModel,
    pub _LeaderboardEntriesAroundPlayerPage_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel,
    >,
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Models";
    const CLASS_NAME: &'static str = "BSLeaderboardAroundMeModel/XocBeatGamesCurrentUserModel/ActiveDataEnvironmentModel/BeatmapModel";
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
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel"
)]
impl std::ops::Deref
for crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel {
    type Target = crate::OculusStudios::GraphQL::Client::GraphQLModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel"
)]
impl std::ops::DerefMut
for crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel"
)]
impl crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel {
    #[cfg(
        feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel"
    )]
    pub type LeaderboardEntriesAroundPlayerPageModel = crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel;
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
    pub fn get_LeaderboardEntriesAroundPlayerPage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel,
                        >,
                        0usize,
                    >("get_LeaderboardEntriesAroundPlayerPage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_LeaderboardEntriesAroundPlayerPage", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_LeaderboardEntriesAroundPlayerPage(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_LeaderboardEntriesAroundPlayerPage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_LeaderboardEntriesAroundPlayerPage", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BSLeaderboardAroundMeModel {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::GraphQLModel,
    pub _XocBeatGamesCurrentUser_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel,
    >,
}
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Models";
    const CLASS_NAME: &'static str = "BSLeaderboardAroundMeModel";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel")]
impl std::ops::Deref for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel {
    type Target = crate::OculusStudios::GraphQL::Client::GraphQLModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel")]
impl std::ops::DerefMut for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel")]
impl crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel {
    #[cfg(
        feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel"
    )]
    pub type XocBeatGamesCurrentUserModel = crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_XocBeatGamesCurrentUser(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel,
                        >,
                        0usize,
                    >("get_XocBeatGamesCurrentUser")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_XocBeatGamesCurrentUser", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_XocBeatGamesCurrentUser(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_XocBeatGamesCurrentUser")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_XocBeatGamesCurrentUser", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel")]
impl quest_hook::libil2cpp::ObjectType
    for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::GraphQLModel,
    pub _ActiveDataEnvironment_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel,
    >,
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Models";
    const CLASS_NAME: &'static str = "BSLeaderboardAroundMeModel/XocBeatGamesCurrentUserModel";
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
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel"
)]
impl std::ops::Deref
for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel {
    type Target = crate::OculusStudios::GraphQL::Client::GraphQLModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel"
)]
impl std::ops::DerefMut
for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel"
)]
impl
    crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel
{
    #[cfg(
        feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel"
    )]
    pub type ActiveDataEnvironmentModel = crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ActiveDataEnvironment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel,
                        >,
                        0usize,
                    >("get_ActiveDataEnvironment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ActiveDataEnvironment", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_ActiveDataEnvironment(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_ActiveDataEnvironment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_ActiveDataEnvironment", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardAroundMeModel_XocBeatGamesCurrentUserModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::GraphQLModel,
    pub _Nodes_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel,
            >,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Models";
    const CLASS_NAME: &'static str = "BSLeaderboardAroundMeModel/XocBeatGamesCurrentUserModel/ActiveDataEnvironmentModel/BeatmapModel/LeaderboardEntriesAroundPlayerPageModel";
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
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel"
)]
impl std::ops::Deref
for crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel {
    type Target = crate::OculusStudios::GraphQL::Client::GraphQLModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel"
)]
impl std::ops::DerefMut
for crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel"
)]
impl crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel {
    #[cfg(
        feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel+NodesModel"
    )]
    pub type NodesModel = crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel;
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
    pub fn get_Nodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_Nodes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Nodes", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Nodes(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Nodes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Nodes", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::GraphQL::Models::BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_LeaderboardEntriesAroundPlayerPageModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel+NodesModel"
)]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel
{
    __cordl_parent: crate::OculusStudios::GraphQL::Client::GraphQLModel,
    pub _LeaderboardEntryWithRank_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntryWithRankModel,
    >,
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel+NodesModel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Models";
    const CLASS_NAME: &'static str = "BSLeaderboardAroundMeModel/XocBeatGamesCurrentUserModel/ActiveDataEnvironmentModel/BeatmapModel/LeaderboardEntriesAroundPlayerPageModel/NodesModel";
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
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel+NodesModel"
)]
impl std::ops::Deref
for crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel {
    type Target = crate::OculusStudios::GraphQL::Client::GraphQLModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel+NodesModel"
)]
impl std::ops::DerefMut
for crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel+NodesModel"
)]
impl crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel {
    pub fn CovertNestedFragments(
        &mut self,
        token: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Linq::JToken,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CovertNestedFragments")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CovertNestedFragments", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (token))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNestedFragment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OculusStudios::GraphQL::Client::GraphQLModel>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::OculusStudios::GraphQL::Client::GraphQLModel,
                        >,
                        0usize,
                    >("GetNestedFragment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNestedFragment", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::GraphQLModel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
    pub fn get_LeaderboardEntryWithRank(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntryWithRankModel,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntryWithRankModel,
                        >,
                        0usize,
                    >("get_LeaderboardEntryWithRank")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_LeaderboardEntryWithRank", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntryWithRankModel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_LeaderboardEntryWithRank(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntryWithRankModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntryWithRankModel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_LeaderboardEntryWithRank")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_LeaderboardEntryWithRank", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel+NodesModel"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel+NodesModel"
)]
impl AsRef<crate::OculusStudios::GraphQL::Client::IFragment>
for crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel {
    fn as_ref(&self) -> &crate::OculusStudios::GraphQL::Client::IFragment {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel+LeaderboardEntriesAroundPlayerPageModel+NodesModel"
)]
impl AsMut<crate::OculusStudios::GraphQL::Client::IFragment>
for crate::BeatSaber::Main::GraphQL::Models::LeaderboardEntriesAroundPlayerPageModel_BeatmapModel_ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_NodesModel {
    fn as_mut(&mut self) -> &mut crate::OculusStudios::GraphQL::Client::IFragment {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::GraphQLModel,
    pub _Beatmap_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel,
    >,
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Models";
    const CLASS_NAME: &'static str = "BSLeaderboardAroundMeModel/XocBeatGamesCurrentUserModel/ActiveDataEnvironmentModel";
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
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel"
)]
impl std::ops::Deref
for crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel {
    type Target = crate::OculusStudios::GraphQL::Client::GraphQLModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel"
)]
impl std::ops::DerefMut
for crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel"
)]
impl crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel {
    #[cfg(
        feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel+BeatmapModel"
    )]
    pub type BeatmapModel = crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel;
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
    pub fn get_Beatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel,
                        >,
                        0usize,
                    >("get_Beatmap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Beatmap", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Beatmap(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::ActiveDataEnvironmentModel_XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_BeatmapModel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Beatmap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Beatmap", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardAroundMeModel+XocBeatGamesCurrentUserModel+ActiveDataEnvironmentModel"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::GraphQL::Models::XocBeatGamesCurrentUserModel_BSLeaderboardAroundMeModel_ActiveDataEnvironmentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
