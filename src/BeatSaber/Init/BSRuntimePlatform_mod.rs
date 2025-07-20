#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
#[repr(C)]
#[derive(Debug)]
pub struct BSRuntimePlatform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Init::BSRuntimePlatform {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Init";
    const CLASS_NAME: &'static str = "BSRuntimePlatform";
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
#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
impl std::ops::Deref for crate::BeatSaber::Init::BSRuntimePlatform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
impl std::ops::DerefMut for crate::BeatSaber::Init::BSRuntimePlatform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
impl crate::BeatSaber::Init::BSRuntimePlatform {
    pub fn GetPlatformType() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Init::RuntimePlatformType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::Init::BSRuntimePlatform as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::BeatSaber::Init::RuntimePlatformType,
                0usize,
            >("GetPlatformType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::Init::BSRuntimePlatform as
                    quest_hook::libil2cpp::Type > ::class(), "GetPlatformType", 0usize
                )
            });
        let __cordl_ret: crate::BeatSaber::Init::RuntimePlatformType = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSteam() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::Init::BSRuntimePlatform as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsSteam")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::Init::BSRuntimePlatform as
                    quest_hook::libil2cpp::Type > ::class(), "get_IsSteam", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+BSRuntimePlatform")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Init::BSRuntimePlatform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
