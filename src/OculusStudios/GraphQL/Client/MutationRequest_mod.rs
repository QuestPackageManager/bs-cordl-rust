#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+MutationRequest")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct MutationRequest {
    __cordl_parent: crate::OculusStudios::GraphQL::ClientInterface::Request,
    pub performRequestCalled: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::OculusStudios::GraphQL::Client::MutationRequest>,
        >,
    >,
    pub mutation:
        quest_hook::libil2cpp::Gc<crate::OculusStudios::GraphQL::Client::GraphQLMutationOperation>,
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+MutationRequest")]
unsafe impl quest_hook::libil2cpp::Type for crate::OculusStudios::GraphQL::Client::MutationRequest {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.Client";
    const CLASS_NAME: &'static str = "MutationRequest";
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
#[cfg(feature = "OculusStudios+GraphQL+Client+MutationRequest")]
impl std::ops::Deref for crate::OculusStudios::GraphQL::Client::MutationRequest {
    type Target = crate::OculusStudios::GraphQL::ClientInterface::Request;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+MutationRequest")]
impl std::ops::DerefMut for crate::OculusStudios::GraphQL::Client::MutationRequest {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+MutationRequest")]
impl crate::OculusStudios::GraphQL::Client::MutationRequest {
    pub fn GetCodeGenInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::ClientInterface::IGraphQLOperation,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::OculusStudios::GraphQL::ClientInterface::IGraphQLOperation,
                    >, 0usize>("GetCodeGenInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCodeGenInfo",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::ClientInterface::IGraphQLOperation,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        mutation: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::GraphQLMutationOperation,
        >,
        forceRequestWhenOffline: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mutation, forceRequestWhenOffline))?;
        Ok(__cordl_object.into())
    }
    pub fn PerformRequest(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PerformRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PerformRequest",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        mutation: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::GraphQLMutationOperation,
        >,
        forceRequestWhenOffline: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::OculusStudios::GraphQL::Client::GraphQLMutationOperation,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (mutation, forceRequestWhenOffline))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_performRequestCalled(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::OculusStudios::GraphQL::Client::MutationRequest>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::GraphQL::Client::MutationRequest,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "add_performRequestCalled"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "add_performRequestCalled",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_performRequestCalled(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::OculusStudios::GraphQL::Client::MutationRequest>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::GraphQL::Client::MutationRequest,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "remove_performRequestCalled"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "remove_performRequestCalled",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+MutationRequest")]
impl quest_hook::libil2cpp::ObjectType for crate::OculusStudios::GraphQL::Client::MutationRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+MutationRequest")]
impl AsRef<crate::OculusStudios::GraphQL::ClientInterface::IMutationRequest>
    for crate::OculusStudios::GraphQL::Client::MutationRequest
{
    fn as_ref(&self) -> &crate::OculusStudios::GraphQL::ClientInterface::IMutationRequest {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+MutationRequest")]
impl AsMut<crate::OculusStudios::GraphQL::ClientInterface::IMutationRequest>
    for crate::OculusStudios::GraphQL::Client::MutationRequest
{
    fn as_mut(&mut self) -> &mut crate::OculusStudios::GraphQL::ClientInterface::IMutationRequest {
        unsafe { std::mem::transmute(self) }
    }
}
