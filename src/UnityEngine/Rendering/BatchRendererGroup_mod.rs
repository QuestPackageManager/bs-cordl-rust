#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct BatchRendererGroup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_GroupHandle: crate::System::IntPtr,
    pub m_PerformCulling: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling,
    >,
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::BatchRendererGroup {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "BatchRendererGroup";
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
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
impl std::ops::Deref for crate::UnityEngine::Rendering::BatchRendererGroup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::BatchRendererGroup {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
impl crate::UnityEngine::Rendering::BatchRendererGroup {
    #[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
    pub type OnPerformCulling = crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling;
    pub fn InvokeOnPerformCulling(
        group: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::BatchRendererGroup,
        >,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::BatchRendererCullingOutput,
        >,
        lodParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LODParameters,
        >,
        userContext: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::BatchRendererGroup,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::BatchRendererCullingOutput,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::LODParameters,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("InvokeOnPerformCulling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InvokeOnPerformCulling", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (group, context, lodParameters, userContext))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::BatchRendererGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
#[repr(C)]
#[derive(Debug)]
pub struct BatchRendererGroup_OnPerformCulling {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "BatchRendererGroup/OnPerformCulling";
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
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
impl crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling {
    pub fn Invoke(
        &mut self,
        rendererGroup: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::BatchRendererGroup,
        >,
        cullingContext: crate::UnityEngine::Rendering::BatchCullingContext,
        cullingOutput: crate::UnityEngine::Rendering::BatchCullingOutput,
        userContext: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::BatchRendererGroup,
                            >,
                            crate::UnityEngine::Rendering::BatchCullingContext,
                            crate::UnityEngine::Rendering::BatchCullingOutput,
                            crate::System::IntPtr,
                        ),
                        crate::Unity::Jobs::JobHandle,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (rendererGroup, cullingContext, cullingOutput, userContext),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererGroup+OnPerformCulling")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::BatchRendererGroup_OnPerformCulling {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
