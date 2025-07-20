#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetBundleExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::AssetBundleExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension";
    const CLASS_NAME: &'static str = "AssetBundleExtensions";
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
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl std::ops::Deref for crate::BGLib::UnityExtension::AssetBundleExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::AssetBundleExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl crate::BGLib::UnityExtension::AssetBundleExtensions {
    pub fn GetAwaiter_AssetBundleCreateRequest0(
        assetBundleCreateRequest: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleCreateRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::AssetBundleCreateRequest,
                        >),
                        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
                        >,
                        1usize,
                    >("GetAwaiter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetAwaiter", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
        > = unsafe { method.invoke_unchecked((), (assetBundleCreateRequest))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAwaiter_ResourceRequest1(
        resourceRequest: quest_hook::libil2cpp::Gc<crate::UnityEngine::ResourceRequest>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::ResourceRequest>),
                        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                        >,
                        1usize,
                    >("GetAwaiter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetAwaiter", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        > = unsafe { method.invoke_unchecked((), (resourceRequest))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::AssetBundleExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
