#[cfg(feature = "cordl_class_InteropErrorExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InteropErrorExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_InteropErrorExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::InteropErrorExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "InteropErrorExtensions";
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
#[cfg(feature = "InteropErrorExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::InteropErrorExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "InteropErrorExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::InteropErrorExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "InteropErrorExtensions")]
impl crate::GlobalNamespace::InteropErrorExtensions {
    pub fn Info(
        error: crate::GlobalNamespace::Interop_Error,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Interop_ErrorInfo> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::Interop_Error),
                        crate::GlobalNamespace::Interop_ErrorInfo,
                        1usize,
                    >("Info")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Info",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::Interop_ErrorInfo =
            unsafe { cordl_method_info.invoke_unchecked((), (error))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_InteropErrorExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::InteropErrorExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
