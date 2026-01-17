#[cfg(feature = "cordl_class_AuthenticationTokenPlatformExtensions")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct AuthenticationTokenPlatformExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_AuthenticationTokenPlatformExtensions")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::AuthenticationTokenPlatformExtensions
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AuthenticationTokenPlatformExtensions";
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
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::AuthenticationTokenPlatformExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::AuthenticationTokenPlatformExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl crate::GlobalNamespace::AuthenticationTokenPlatformExtensions {
    pub fn ToAuthenticationTokenPlatform(
        platform: crate::GlobalNamespace::UserInfo_Platform,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::AuthenticationToken_PlatformType>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::UserInfo_Platform),
                        crate::GlobalNamespace::AuthenticationToken_PlatformType,
                        1usize,
                    >("ToAuthenticationTokenPlatform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToAuthenticationTokenPlatform", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::AuthenticationToken_PlatformType =
            unsafe { cordl_method_info.invoke_unchecked((), (platform))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_AuthenticationTokenPlatformExtensions")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::AuthenticationTokenPlatformExtensions
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
