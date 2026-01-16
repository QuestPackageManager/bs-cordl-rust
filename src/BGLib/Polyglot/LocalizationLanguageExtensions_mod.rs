#[cfg(feature = "cordl_class_BGLib+Polyglot+LocalizationLanguageExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizationLanguageExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BGLib+Polyglot+LocalizationLanguageExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::BGLib::Polyglot::LocalizationLanguageExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.Polyglot";
    const CLASS_NAME: &'static str = "LocalizationLanguageExtensions";
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
#[cfg(feature = "BGLib+Polyglot+LocalizationLanguageExtensions")]
impl std::ops::Deref for crate::BGLib::Polyglot::LocalizationLanguageExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationLanguageExtensions")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LocalizationLanguageExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationLanguageExtensions")]
impl crate::BGLib::Polyglot::LocalizationLanguageExtensions {
    pub fn GetLanguageDirection(
        language: crate::BGLib::Polyglot::LocalizationLanguage,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::LanguageDirection> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BGLib::Polyglot::LocalizationLanguage),
                        crate::BGLib::Polyglot::LanguageDirection,
                        1usize,
                    >("GetLanguageDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLanguageDirection", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::BGLib::Polyglot::LanguageDirection =
            unsafe { cordl_method_info.invoke_unchecked((), (language))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToCultureInfoName(
        language: crate::BGLib::Polyglot::LocalizationLanguage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BGLib::Polyglot::LocalizationLanguage),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("ToCultureInfoName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToCultureInfoName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (language))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToLocalizationLanguage_Il2CppString0(
        serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::LocalizationLanguage> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::BGLib::Polyglot::LocalizationLanguage,
                        1usize,
                    >("ToLocalizationLanguage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToLocalizationLanguage", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::BGLib::Polyglot::LocalizationLanguage =
            unsafe { cordl_method_info.invoke_unchecked((), (serializedName))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToLocalizationLanguage_SystemLanguage1(
        systemLanguage: crate::UnityEngine::SystemLanguage,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<bool, crate::BGLib::Polyglot::LocalizationLanguage>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::SystemLanguage),
                        crate::System::ValueTuple_2<
                            bool,
                            crate::BGLib::Polyglot::LocalizationLanguage,
                        >,
                        1usize,
                    >("ToLocalizationLanguage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToLocalizationLanguage", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            bool,
            crate::BGLib::Polyglot::LocalizationLanguage,
        > = unsafe { cordl_method_info.invoke_unchecked((), (systemLanguage))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToSerializedName(
        language: crate::BGLib::Polyglot::LocalizationLanguage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::BGLib::Polyglot::LocalizationLanguage),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("ToSerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToSerializedName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (language))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BGLib+Polyglot+LocalizationLanguageExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LocalizationLanguageExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
