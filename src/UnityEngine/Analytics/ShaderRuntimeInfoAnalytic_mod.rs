#[cfg(feature = "cordl_class_UnityEngine+Analytics+ShaderRuntimeInfoAnalytic")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderRuntimeInfoAnalytic {
    __cordl_parent: crate::UnityEngine::Analytics::AnalyticsEventBase,
    pub VariantsRequested: i64,
    pub VariantsRequestedMissing: i64,
    pub VariantsRequestedUnsupported: i64,
    pub VariantsRequestedCompiled: i64,
    pub VariantsRequestedViaWarmup: i64,
    pub VariantsUnused: i64,
    pub VariantsCompilationTimeTotal: i32,
    pub VariantsCompilationTimeMax: i32,
    pub VariantsCompilationTimeMedian: i32,
    pub VariantsWarmupTimeTotal: i32,
    pub VariantsWarmupTimeMax: i32,
    pub VariantsWarmupTimeMedian: i32,
    pub UseProgressiveWarmup: bool,
    pub ShaderChunkSizeMin: i32,
    pub ShaderChunkSizeMax: i32,
    pub ShaderChunkSizeAvg: i32,
    pub ShaderChunkCountMin: i32,
    pub ShaderChunkCountMax: i32,
    pub ShaderChunkCountAvg: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Analytics+ShaderRuntimeInfoAnalytic")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Analytics::ShaderRuntimeInfoAnalytic {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Analytics";
    const CLASS_NAME: &'static str = "ShaderRuntimeInfoAnalytic";
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
#[cfg(feature = "UnityEngine+Analytics+ShaderRuntimeInfoAnalytic")]
impl std::ops::Deref for crate::UnityEngine::Analytics::ShaderRuntimeInfoAnalytic {
    type Target = crate::UnityEngine::Analytics::AnalyticsEventBase;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Analytics+ShaderRuntimeInfoAnalytic")]
impl std::ops::DerefMut for crate::UnityEngine::Analytics::ShaderRuntimeInfoAnalytic {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Analytics+ShaderRuntimeInfoAnalytic")]
impl crate::UnityEngine::Analytics::ShaderRuntimeInfoAnalytic {
    pub fn CreateShaderRuntimeInfoAnalytic() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Analytics::ShaderRuntimeInfoAnalytic,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Analytics::ShaderRuntimeInfoAnalytic,
                        >,
                        0usize,
                    >("CreateShaderRuntimeInfoAnalytic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateShaderRuntimeInfoAnalytic", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Analytics::ShaderRuntimeInfoAnalytic,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Analytics+ShaderRuntimeInfoAnalytic")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Analytics::ShaderRuntimeInfoAnalytic {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
