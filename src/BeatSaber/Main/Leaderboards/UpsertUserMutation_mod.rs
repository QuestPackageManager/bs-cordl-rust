#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+UpsertUserMutation")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct UpsertUserMutation {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::MutationRequest_1<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData,
        >,
    >,
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+UpsertUserMutation")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::Main::Leaderboards::UpsertUserMutation
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.Leaderboards";
    const CLASS_NAME: &'static str = "UpsertUserMutation";
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
#[cfg(feature = "BeatSaber+Main+Leaderboards+UpsertUserMutation")]
impl std::ops::Deref for crate::BeatSaber::Main::Leaderboards::UpsertUserMutation {
    type Target = crate::OculusStudios::GraphQL::Client::MutationRequest_1<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData,
        >,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+UpsertUserMutation")]
impl std::ops::DerefMut for crate::BeatSaber::Main::Leaderboards::UpsertUserMutation {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+Leaderboards+UpsertUserMutation")]
impl crate::BeatSaber::Main::Leaderboards::UpsertUserMutation {
    pub fn New(
        settings: crate::BeatSaber::Settings::Settings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (settings))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        settings: crate::BeatSaber::Settings::Settings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::Settings::Settings),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (settings))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Main+Leaderboards+UpsertUserMutation")]
impl quest_hook::libil2cpp::ObjectType
    for crate::BeatSaber::Main::Leaderboards::UpsertUserMutation
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
