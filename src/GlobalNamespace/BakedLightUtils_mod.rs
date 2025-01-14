#[cfg(feature = "BakedLightUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct BakedLightUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BakedLightUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BakedLightUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BakedLightUtils";
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
#[cfg(feature = "BakedLightUtils")]
impl std::ops::Deref for crate::GlobalNamespace::BakedLightUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BakedLightUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::BakedLightUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BakedLightUtils")]
impl crate::GlobalNamespace::BakedLightUtils {
    pub const kDepthOnlyShaderName: &'static str = "Custom/SetDepthOnly";
    pub const kMirrorParentNameToIgnore: &'static str = "PlayersPlace";
    pub fn ValidateLoadedEnvironmentScene(
        validateBakedGIEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ValidateLoadedEnvironmentScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ValidateLoadedEnvironmentScene", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (validateBakedGIEnabled))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BakedLightUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BakedLightUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
