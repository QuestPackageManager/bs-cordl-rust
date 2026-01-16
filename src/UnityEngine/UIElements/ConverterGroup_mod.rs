#[cfg(feature = "cordl_class_UnityEngine+UIElements+ConverterGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct ConverterGroup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _displayName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _description_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _registry_k__BackingField: crate::UnityEngine::UIElements::TypeConverterRegistry,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+ConverterGroup")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::ConverterGroup {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "ConverterGroup";
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
#[cfg(feature = "UnityEngine+UIElements+ConverterGroup")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ConverterGroup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ConverterGroup")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ConverterGroup {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ConverterGroup")]
impl crate::UnityEngine::UIElements::ConverterGroup {
    pub fn New(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id, displayName, description))?;
        Ok(__cordl_object.into())
    }
    pub fn TryConvert<TSource, TDestination>(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<TSource>,
        destination: quest_hook::libil2cpp::ByRefMut<TDestination>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TDestination: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TSource>,
                            quest_hook::libil2cpp::ByRefMut<TDestination>,
                        ),
                        bool,
                        2usize,
                    >("TryConvert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryConvert", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (source, destination))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrySetValue<TContainer, TValue>(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        path: quest_hook::libil2cpp::ByRefMut<crate::Unity::Properties::PropertyPath>,
        value: TValue,
        returnCode: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Properties::VisitReturnCode,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Properties::PropertyPath,
                            >,
                            TValue,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Properties::VisitReturnCode,
                            >,
                        ),
                        bool,
                        4usize,
                    >("TrySetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrySetValue", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (container, path, value, returnCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (id, displayName, description))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_registry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TypeConverterRegistry,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::TypeConverterRegistry,
                        0usize,
                    >("get_registry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_registry", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::TypeConverterRegistry = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+ConverterGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ConverterGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
