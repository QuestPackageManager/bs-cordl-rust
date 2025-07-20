#[cfg(feature = "OculusRequestExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusRequestExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OculusRequestExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusRequestExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusRequestExtensions";
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
#[cfg(feature = "OculusRequestExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OculusRequestExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusRequestExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusRequestExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusRequestExtensions")]
impl crate::GlobalNamespace::OculusRequestExtensions {
    pub fn ToPlatformException(
        error: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
        errorType: crate::GlobalNamespace::PlatformException_ErrorType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlatformException>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Oculus::Platform::Models::Error,
                            >,
                            crate::GlobalNamespace::PlatformException_ErrorType,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::PlatformException,
                        >,
                        2usize,
                    >("ToPlatformException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToPlatformException", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformException,
        > = unsafe { method.invoke_unchecked((), (error, errorType))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusRequestExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusRequestExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
