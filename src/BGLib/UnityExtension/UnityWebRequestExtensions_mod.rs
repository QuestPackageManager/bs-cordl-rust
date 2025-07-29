#[cfg(feature = "cordl_class_BGLib+UnityExtension+UnityWebRequestExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BGLib+UnityExtension+UnityWebRequestExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.UnityExtension";
    const CLASS_NAME: &'static str = "UnityWebRequestExtensions";
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
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl std::ops::Deref for crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    pub fn GetAwaiter(
        webRequestOperation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            crate::UnityEngine::Networking::UnityWebRequest_Result,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
                        >),
                        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
                            crate::UnityEngine::Networking::UnityWebRequest_Result,
                        >,
                        1usize,
                    >("GetAwaiter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAwaiter", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            crate::UnityEngine::Networking::UnityWebRequest_Result,
        > = unsafe { cordl_method_info.invoke_unchecked((), (webRequestOperation))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendWebRequestAsync(
        request: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest_Result>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Networking::UnityWebRequest,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Networking::UnityWebRequest_Result,
                        >,
                        2usize,
                    >("SendWebRequestAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendWebRequestAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest_Result,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (request, cancellationToken))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BGLib+UnityExtension+UnityWebRequestExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
