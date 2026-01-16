#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XOCBeatGamesUserUpsertData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _params: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::SortedDictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Main.GraphQL.InputTypes";
    const CLASS_NAME: &'static str = "XOCBeatGamesUserUpsertData";
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
#[cfg(feature = "BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData")]
impl std::ops::Deref
for crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData")]
impl std::ops::DerefMut
for crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData")]
impl crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    pub fn GetParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
                            crate::System::Collections::Generic::IDictionary_2<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >,
                        0usize,
                    >("GetParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetParams", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
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
    pub fn set_CountryCode(
        &mut self,
        value: crate::System::Nullable_1<
            crate::BeatSaber::Main::GraphQL::Enums::ISOCountryCode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Nullable_1<
                            crate::BeatSaber::Main::GraphQL::Enums::ISOCountryCode,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_CountryCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_CountryCode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Locale(
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
                    >("set_Locale")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Locale", 1usize
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
    feature = "cordl_class_BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData")]
impl AsRef<crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesUserUpsertData>
for crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    fn as_ref(
        &self,
    ) -> &crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesUserUpsertData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData")]
impl AsMut<crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesUserUpsertData>
for crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::BeatSaber::Main::GraphQL::InputTypes::IXOCBeatGamesUserUpsertData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData")]
impl AsRef<crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputObject>
for crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    fn as_ref(
        &self,
    ) -> &crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputObject {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData")]
impl AsMut<crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputObject>
for crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputObject {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData")]
impl AsRef<
    crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputSupportsClientMutationId,
> for crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    fn as_ref(
        &self,
    ) -> &crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputSupportsClientMutationId {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Main+GraphQL+InputTypes+XOCBeatGamesUserUpsertData")]
impl AsMut<
    crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputSupportsClientMutationId,
> for crate::BeatSaber::Main::GraphQL::InputTypes::XOCBeatGamesUserUpsertData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::OculusStudios::GraphQL::ClientInterface::IGraphQLInputSupportsClientMutationId {
        unsafe { std::mem::transmute(self) }
    }
}
