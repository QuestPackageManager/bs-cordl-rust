#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCachedChunk")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct DecalCachedChunk {
    __cordl_parent: crate::UnityEngine::Rendering::Universal::DecalChunk,
    pub propertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    pub passIndexDBuffer: i32,
    pub passIndexEmissive: i32,
    pub passIndexScreenSpace: i32,
    pub passIndexGBuffer: i32,
    pub drawOrder: i32,
    pub isCreated: bool,
    pub decalToWorlds:
        crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float4x4>,
    pub normalToWorlds:
        crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float4x4>,
    pub sizeOffsets: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float4x4>,
    pub drawDistances: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float2>,
    pub angleFades: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float2>,
    pub uvScaleBias: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float4>,
    pub layerMasks: crate::Unity::Collections::NativeArray_1<i32>,
    pub sceneLayerMasks: crate::Unity::Collections::NativeArray_1<u64>,
    pub fadeFactors: crate::Unity::Collections::NativeArray_1<f32>,
    pub boundingSpheres:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::BoundingSphere>,
    pub scaleModes: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::Universal::DecalScaleMode,
    >,
    pub renderingLayerMasks: crate::Unity::Collections::NativeArray_1<u32>,
    pub positions: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float3>,
    pub rotation: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::quaternion>,
    pub scales: crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float3>,
    pub dirty: crate::Unity::Collections::NativeArray_1<bool>,
    pub boundingSphereArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoundingSphere>,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCachedChunk")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::DecalCachedChunk
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DecalCachedChunk";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalCachedChunk")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::DecalCachedChunk {
    type Target = crate::UnityEngine::Rendering::Universal::DecalChunk;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalCachedChunk")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::DecalCachedChunk {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalCachedChunk")]
impl crate::UnityEngine::Rendering::Universal::DecalCachedChunk {
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveAtSwapBack(
        &mut self,
        entityIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("RemoveAtSwapBack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RemoveAtSwapBack",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (entityIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetCapacity(
        &mut self,
        newCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("SetCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetCapacity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (newCapacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalCachedChunk")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::DecalCachedChunk
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
