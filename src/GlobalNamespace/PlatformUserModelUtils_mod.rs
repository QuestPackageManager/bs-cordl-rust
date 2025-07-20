#[cfg(feature = "PlatformUserModelUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformUserModelUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PlatformUserModelUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformUserModelUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformUserModelUtils";
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
#[cfg(feature = "PlatformUserModelUtils")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformUserModelUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformUserModelUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformUserModelUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformUserModelUtils")]
impl crate::GlobalNamespace::PlatformUserModelUtils {
    pub const kMinimalTokenLength: i32 = 64i32;
    pub fn ValidateXPlatformAccessToken(
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlatformUserModelUtils as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("ValidateXPlatformAccessToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::PlatformUserModelUtils as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ValidateXPlatformAccessToken", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (token))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlatformUserModelUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformUserModelUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
