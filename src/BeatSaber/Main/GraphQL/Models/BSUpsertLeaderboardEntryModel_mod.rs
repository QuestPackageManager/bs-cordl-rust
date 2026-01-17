#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct BSUpsertLeaderboardEntryModel {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::GraphQLModel,
    pub _XocBeatGamesBeatmapLeaderboardEntryUpsert_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel,
    >,
}
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Models";
    const CLASS_NAME: &'static str = "BSUpsertLeaderboardEntryModel";
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
#[cfg(feature = "BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel")]
impl std::ops::Deref for crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel {
    type Target = crate::OculusStudios::GraphQL::Client::GraphQLModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel")]
impl std::ops::DerefMut for crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel")]
impl crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel {
    #[cfg(
        feature = "BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel+XocBeatGamesBeatmapLeaderboardEntryUpsertModel"
    )]
    pub type XocBeatGamesBeatmapLeaderboardEntryUpsertModel = crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel;
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
    pub fn get_XocBeatGamesBeatmapLeaderboardEntryUpsert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel,
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
                            crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel,
                        >,
                        0usize,
                    >("get_XocBeatGamesBeatmapLeaderboardEntryUpsert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_XocBeatGamesBeatmapLeaderboardEntryUpsert", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_XocBeatGamesBeatmapLeaderboardEntryUpsert(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_XocBeatGamesBeatmapLeaderboardEntryUpsert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_XocBeatGamesBeatmapLeaderboardEntryUpsert", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel")]
impl quest_hook::libil2cpp::ObjectType
    for crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel+XocBeatGamesBeatmapLeaderboardEntryUpsertModel"
)]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::GraphQLModel,
    pub _ClientMutationId_k__BackingField:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel+XocBeatGamesBeatmapLeaderboardEntryUpsertModel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Models";
    const CLASS_NAME: &'static str = "BSUpsertLeaderboardEntryModel/XocBeatGamesBeatmapLeaderboardEntryUpsertModel";
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
    feature = "BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel+XocBeatGamesBeatmapLeaderboardEntryUpsertModel"
)]
impl std::ops::Deref
for crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel {
    type Target = crate::OculusStudios::GraphQL::Client::GraphQLModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel+XocBeatGamesBeatmapLeaderboardEntryUpsertModel"
)]
impl std::ops::DerefMut
for crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel+XocBeatGamesBeatmapLeaderboardEntryUpsertModel"
)]
impl crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel {
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
    pub fn get_ClientMutationId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_ClientMutationId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ClientMutationId", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_ClientMutationId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_ClientMutationId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_ClientMutationId", 1usize
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
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSUpsertLeaderboardEntryModel+XocBeatGamesBeatmapLeaderboardEntryUpsertModel"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::GraphQL::Models::BSUpsertLeaderboardEntryModel_XocBeatGamesBeatmapLeaderboardEntryUpsertModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
