#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+IMutationRequest")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct IMutationRequest {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+IMutationRequest")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::OculusStudios::GraphQL::ClientInterface::IMutationRequest
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.ClientInterface";
    const CLASS_NAME: &'static str = "IMutationRequest";
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
#[cfg(feature = "OculusStudios+GraphQL+ClientInterface+IMutationRequest")]
impl std::ops::Deref for crate::OculusStudios::GraphQL::ClientInterface::IMutationRequest {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+ClientInterface+IMutationRequest")]
impl std::ops::DerefMut for crate::OculusStudios::GraphQL::ClientInterface::IMutationRequest {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+ClientInterface+IMutationRequest")]
impl crate::OculusStudios::GraphQL::ClientInterface::IMutationRequest {
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+IMutationRequest")]
impl quest_hook::libil2cpp::ObjectType
    for crate::OculusStudios::GraphQL::ClientInterface::IMutationRequest
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
