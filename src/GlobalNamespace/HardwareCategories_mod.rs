#[cfg(feature = "cordl_class_HardwareCategories")]
#[repr(C)]
#[derive(Debug)]
pub struct HardwareCategories {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_HardwareCategories")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::HardwareCategories {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "HardwareCategories";
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
#[cfg(feature = "HardwareCategories")]
impl std::ops::Deref for crate::GlobalNamespace::HardwareCategories {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HardwareCategories")]
impl std::ops::DerefMut for crate::GlobalNamespace::HardwareCategories {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HardwareCategories")]
impl crate::GlobalNamespace::HardwareCategories {
    pub fn GetHardwareCategory(
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::HardwareCategory> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), crate::GlobalNamespace::HardwareCategory, 0usize>(
                        "GetHardwareCategory",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHardwareCategory",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::HardwareCategory =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHardwareCategoryWithEditorOverride(
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::HardwareCategory> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), crate::GlobalNamespace::HardwareCategory, 0usize>(
                        "GetHardwareCategoryWithEditorOverride",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHardwareCategoryWithEditorOverride",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::HardwareCategory =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_HardwareCategories")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::HardwareCategories {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
