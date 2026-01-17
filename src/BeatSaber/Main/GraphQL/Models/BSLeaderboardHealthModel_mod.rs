#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct BSLeaderboardHealthModel {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::GraphQLModel,
    pub _XocBeatGamesLeaderboardHealth_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel,
    >,
}
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Models";
    const CLASS_NAME: &'static str = "BSLeaderboardHealthModel";
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
#[cfg(feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel")]
impl std::ops::Deref for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel {
    type Target = crate::OculusStudios::GraphQL::Client::GraphQLModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel")]
impl std::ops::DerefMut for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel")]
impl crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel {
    #[cfg(
        feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel+XocBeatGamesLeaderboardHealthModel"
    )]
    pub type XocBeatGamesLeaderboardHealthModel = crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel;
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
    pub fn get_XocBeatGamesLeaderboardHealth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel,
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
                            crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel,
                        >,
                        0usize,
                    >("get_XocBeatGamesLeaderboardHealth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_XocBeatGamesLeaderboardHealth", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_XocBeatGamesLeaderboardHealth(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_XocBeatGamesLeaderboardHealth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_XocBeatGamesLeaderboardHealth", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel")]
impl quest_hook::libil2cpp::ObjectType
    for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel+XocBeatGamesLeaderboardHealthModel"
)]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::GraphQLModel,
    pub _Status_k__BackingField: crate::System::Nullable_1<
        crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesHealthCheckStatusValue,
    >,
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel+XocBeatGamesLeaderboardHealthModel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.Models";
    const CLASS_NAME: &'static str = "BSLeaderboardHealthModel/XocBeatGamesLeaderboardHealthModel";
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
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel+XocBeatGamesLeaderboardHealthModel"
)]
impl std::ops::Deref
for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel {
    type Target = crate::OculusStudios::GraphQL::Client::GraphQLModel;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel+XocBeatGamesLeaderboardHealthModel"
)]
impl std::ops::DerefMut
for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel+XocBeatGamesLeaderboardHealthModel"
)]
impl crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel {
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
    pub fn get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesHealthCheckStatusValue,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Nullable_1<
                            crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesHealthCheckStatusValue,
                        >,
                        0usize,
                    >("get_Status")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Status", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesHealthCheckStatusValue,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Status(
        &mut self,
        value: crate::System::Nullable_1<
            crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesHealthCheckStatusValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Nullable_1<
                            crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesHealthCheckStatusValue,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Status")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Status", 1usize
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
    feature = "cordl_class_BeatSaber+Main+GraphQL+Models+BSLeaderboardHealthModel+XocBeatGamesLeaderboardHealthModel"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::GraphQL::Models::BSLeaderboardHealthModel_XocBeatGamesLeaderboardHealthModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
