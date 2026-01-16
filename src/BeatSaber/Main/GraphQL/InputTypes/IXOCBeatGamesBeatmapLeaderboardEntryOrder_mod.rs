#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+InputTypes+IXOCBeatGamesBeatmapLeaderboardEntryOrder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct IXOCBeatGamesBeatmapLeaderboardEntryOrder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+InputTypes+IXOCBeatGamesBeatmapLeaderboardEntryOrder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesBeatmapLeaderboardEntryOrder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.InputTypes";
    const CLASS_NAME: &'static str = "IXOCBeatGamesBeatmapLeaderboardEntryOrder";
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
    feature = "BeatSaber+Main+GraphQL+InputTypes+IXOCBeatGamesBeatmapLeaderboardEntryOrder"
)]
impl std::ops::Deref
for crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesBeatmapLeaderboardEntryOrder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+InputTypes+IXOCBeatGamesBeatmapLeaderboardEntryOrder"
)]
impl std::ops::DerefMut
for crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesBeatmapLeaderboardEntryOrder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+InputTypes+IXOCBeatGamesBeatmapLeaderboardEntryOrder"
)]
impl crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesBeatmapLeaderboardEntryOrder {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn set_By(
        &mut self,
        value: crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapLeaderboardEntryOrderBySubject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::Main::GraphQL::Enums::XOCBeatGamesBeatmapLeaderboardEntryOrderBySubject),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_By")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "set_By",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Desc(
        &mut self,
        value: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Nullable_1<bool>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Desc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Desc", 1usize
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
    feature = "cordl_class_BeatSaber+Main+GraphQL+InputTypes+IXOCBeatGamesBeatmapLeaderboardEntryOrder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesBeatmapLeaderboardEntryOrder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+InputTypes+IXOCBeatGamesBeatmapLeaderboardEntryOrder"
)]
impl AsRef<crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputObject>
for crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesBeatmapLeaderboardEntryOrder {
    fn as_ref(
        &self,
    ) -> &crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputObject {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "BeatSaber+Main+GraphQL+InputTypes+IXOCBeatGamesBeatmapLeaderboardEntryOrder"
)]
impl AsMut<crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputObject>
for crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesBeatmapLeaderboardEntryOrder {
    fn as_mut(
        &mut self,
    ) -> &mut crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputObject {
        unsafe { std::mem::transmute(self) }
    }
}
