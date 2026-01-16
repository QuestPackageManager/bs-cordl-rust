#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct DecalCreateDrawCallSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_EntityManager: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::DecalEntityManager,
    >,
    pub m_Sampler: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ProfilingSampler,
    >,
    pub m_MaxDrawDistance: f32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DecalCreateDrawCallSystem";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem")]
impl crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem {
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob"
    )]
    pub type DrawCallJob = crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem_DrawCallJob;
    pub fn Execute_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_DecalCachedChunk_DecalCulledChunk_DecalDrawCallChunk_i32_1(
        &mut self,
        cachedChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
        >,
        culledChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalCulledChunk,
        >,
        drawCallChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::DecalCulledChunk,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (cachedChunk, culledChunk, drawCallChunk, count),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        entityManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityManager,
        >,
        maxDrawDistance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entityManager, maxDrawDistance))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        entityManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityManager,
        >,
        maxDrawDistance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::DecalEntityManager,
                            >,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (entityManager, maxDrawDistance))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxDrawDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_maxDrawDistance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_maxDrawDistance", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_maxDrawDistance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_maxDrawDistance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_maxDrawDistance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecalCreateDrawCallSystem_DrawCallJob {
    pub decalToWorlds: crate::Unity::Collections::NativeArray_1<
        crate::Unity::Mathematics::float4x4,
    >,
    pub normalToWorlds: crate::Unity::Collections::NativeArray_1<
        crate::Unity::Mathematics::float4x4,
    >,
    pub sizeOffsets: crate::Unity::Collections::NativeArray_1<
        crate::Unity::Mathematics::float4x4,
    >,
    pub drawDistances: crate::Unity::Collections::NativeArray_1<
        crate::Unity::Mathematics::float2,
    >,
    pub angleFades: crate::Unity::Collections::NativeArray_1<
        crate::Unity::Mathematics::float2,
    >,
    pub uvScaleBiases: crate::Unity::Collections::NativeArray_1<
        crate::Unity::Mathematics::float4,
    >,
    pub layerMasks: crate::Unity::Collections::NativeArray_1<i32>,
    pub sceneLayerMasks: crate::Unity::Collections::NativeArray_1<u64>,
    pub fadeFactors: crate::Unity::Collections::NativeArray_1<f32>,
    pub boundingSpheres: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::BoundingSphere,
    >,
    pub renderingLayerMasks: crate::Unity::Collections::NativeArray_1<u32>,
    pub cameraPosition: crate::UnityEngine::Vector3,
    pub sceneCullingMask: u64,
    pub cullingMask: i32,
    pub visibleDecalIndices: crate::Unity::Collections::NativeArray_1<i32>,
    pub visibleDecalCount: i32,
    pub maxDrawDistance: f32,
    pub decalToWorldsDraw: crate::Unity::Collections::NativeArray_1<
        crate::Unity::Mathematics::float4x4,
    >,
    pub normalToDecalsDraw: crate::Unity::Collections::NativeArray_1<
        crate::Unity::Mathematics::float4x4,
    >,
    pub renderingLayerMasksDraw: crate::Unity::Collections::NativeArray_1<f32>,
    pub subCalls: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::Universal::DecalSubDrawCall,
    >,
    pub subCallCount: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem_DrawCallJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DecalCreateDrawCallSystem/DrawCallJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem_DrawCallJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem_DrawCallJob {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem_DrawCallJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem_DrawCallJob {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem_DrawCallJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob")]
impl crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem_DrawCallJob {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
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
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob")]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem_DrawCallJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalCreateDrawCallSystem+DrawCallJob")]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::Universal::DecalCreateDrawCallSystem_DrawCallJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
