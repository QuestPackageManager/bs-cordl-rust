#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+GraphErrorCodeExtensions")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct GraphErrorCodeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+GraphErrorCodeExtensions")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCodeExtensions
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.ClientInterface";
    const CLASS_NAME: &'static str = "GraphErrorCodeExtensions";
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
#[cfg(feature = "OculusStudios+GraphQL+ClientInterface+GraphErrorCodeExtensions")]
impl std::ops::Deref for crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCodeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+ClientInterface+GraphErrorCodeExtensions")]
impl std::ops::DerefMut
    for crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCodeExtensions
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+ClientInterface+GraphErrorCodeExtensions")]
impl crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCodeExtensions {
    pub fn GetGraphQLErrorCode(
        code: i32,
    ) -> quest_hook::libil2cpp::Result<crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCode>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCode,
                        1usize,
                    >("GetGraphQLErrorCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGraphQLErrorCode", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCode =
            unsafe { cordl_method_info.invoke_unchecked((), (code))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsGraphQLErrorCode(code: i32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), bool, 1usize>("IsGraphQLErrorCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsGraphQLErrorCode",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (code))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+GraphErrorCodeExtensions")]
impl quest_hook::libil2cpp::ObjectType
    for crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCodeExtensions
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
