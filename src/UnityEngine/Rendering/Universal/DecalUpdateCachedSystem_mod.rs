#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalUpdateCachedSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct DecalUpdateCachedSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_EntityManager:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::DecalEntityManager>,
    pub m_Sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
    pub m_SamplerJob: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalUpdateCachedSystem")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DecalUpdateCachedSystem";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalUpdateCachedSystem")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalUpdateCachedSystem")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalUpdateCachedSystem")]
impl crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem {
    #[cfg(feature = "UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob")]
    pub type UpdateTransformsJob =
        crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem_UpdateTransformsJob;
    pub fn Execute_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_DecalEntityChunk_DecalCachedChunk_i32_1(
        &mut self,
        entityChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
        >,
        cachedChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (entityChunk, cachedChunk, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        entityManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entityManager))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        entityManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::DecalEntityManager,
                    >), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (entityManager))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalUpdateCachedSystem")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct DecalUpdateCachedSystem_UpdateTransformsJob {
    pub positions: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float3>,
    pub rotations: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::quaternion>,
    pub scales: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float3>,
    pub dirty: crate::Unity::Collections::NativeArray_1<bool>,
    pub scaleModes: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::Universal::DecalScaleMode,
    >,
    pub sizeOffsets: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float4x4>,
    pub decalToWorlds:
        crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float4x4>,
    pub normalToWorlds:
        crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float4x4>,
    pub boundingSpheres:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::BoundingSphere>,
    pub minDistance: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem_UpdateTransformsJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DecalUpdateCachedSystem/UpdateTransformsJob";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem_UpdateTransformsJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem_UpdateTransformsJob
{
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem_UpdateTransformsJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem_UpdateTransformsJob
{
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem_UpdateTransformsJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob")]
impl crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem_UpdateTransformsJob {
    pub fn DistanceBetweenQuaternions(
        &mut self,
        a: crate::Unity::Mathematics::quaternion,
        b: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Mathematics::quaternion,
                        crate::Unity::Mathematics::quaternion,
                    ), f32, 2usize>("DistanceBetweenQuaternions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DistanceBetweenQuaternions",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        index: i32,
        transform: crate::UnityEngine::Jobs::TransformAccess,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Jobs::TransformAccess),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index, transform))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDecalProjectBoundingSphere(
        &mut self,
        decalToWorld: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::BoundingSphere> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Matrix4x4),
                        crate::UnityEngine::BoundingSphere,
                        1usize,
                    >("GetDecalProjectBoundingSphere")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDecalProjectBoundingSphere", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::BoundingSphere =
            unsafe { cordl_method_info.invoke_unchecked(self, (decalToWorld))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob")]
impl AsRef<crate::UnityEngine::Jobs::IJobParallelForTransform>
    for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem_UpdateTransformsJob
{
    fn as_ref(&self) -> &crate::UnityEngine::Jobs::IJobParallelForTransform {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalUpdateCachedSystem+UpdateTransformsJob")]
impl AsMut<crate::UnityEngine::Jobs::IJobParallelForTransform>
    for crate::UnityEngine::Rendering::Universal::DecalUpdateCachedSystem_UpdateTransformsJob
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Jobs::IJobParallelForTransform {
        todo!()
    }
}
