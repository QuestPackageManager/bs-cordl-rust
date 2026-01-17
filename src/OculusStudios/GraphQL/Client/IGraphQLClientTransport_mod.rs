#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+IGraphQLClientTransport")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct IGraphQLClientTransport {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+IGraphQLClientTransport")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.Client";
    const CLASS_NAME: &'static str = "IGraphQLClientTransport";
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
#[cfg(feature = "OculusStudios+GraphQL+Client+IGraphQLClientTransport")]
impl std::ops::Deref for crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+IGraphQLClientTransport")]
impl std::ops::DerefMut for crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+IGraphQLClientTransport")]
impl crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport {
    pub fn ExecuteAsync(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::OculusStudios::GraphQL::Client::GraphQLRequest>,
        mainThreadExecutor: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::MinimalMainThreadExecutor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::OculusStudios::GraphQL::Client::GraphQLResponseStream,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::OculusStudios::GraphQL::Client::GraphQLRequest,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::OculusStudios::GraphQL::Client::MinimalMainThreadExecutor,
                        >,
                    ), quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::Task_1<
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::GraphQL::Client::GraphQLResponseStream,
                            >,
                        >,
                    >, 2usize>("ExecuteAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::OculusStudios::GraphQL::Client::GraphQLResponseStream,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (request, mainThreadExecutor))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+IGraphQLClientTransport")]
impl quest_hook::libil2cpp::ObjectType
    for crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+IGraphQLClientTransport")]
impl AsRef<crate::System::IDisposable>
    for crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+IGraphQLClientTransport")]
impl AsMut<crate::System::IDisposable>
    for crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
