#[cfg(feature = "UnityEngine+Graphics")]
#[repr(C)]
#[derive(Debug)]
pub struct Graphics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Graphics")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Graphics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Graphics";
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
#[cfg(feature = "UnityEngine+Graphics")]
impl std::ops::Deref for crate::UnityEngine::Graphics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Graphics")]
impl std::ops::DerefMut for crate::UnityEngine::Graphics {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Graphics")]
impl crate::UnityEngine::Graphics {
    pub fn Blit2(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Blit2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Blit2",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (source, dest))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit4(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Blit4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Blit4",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (source, dest, scale, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit4_Injected(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Blit4_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Blit4_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (source, dest, scale, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Material3(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Blit",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (source, dest, mat))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Material_i32_2(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Blit",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (source, dest, mat, pass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Texture_RenderTexture0(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Blit",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (source, dest))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Vector2_Vector2_1(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Blit",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (source, dest, scale, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_Full(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CopyTexture_Full")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyTexture_Full", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (src, dst))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_Slice(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        srcElement: i32,
        srcMip: i32,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dstElement: i32,
        dstMip: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("CopyTexture_Slice")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyTexture_Slice", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (src, srcElement, srcMip, dst, dstElement, dstMip),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_Texture0(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CopyTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyTexture", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (src, dst))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_i32_i32_Texture_i32_i32_1(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        srcElement: i32,
        srcMip: i32,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dstElement: i32,
        dstMip: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("CopyTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyTexture", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (src, srcElement, srcMip, dst, dstElement, dstMip),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced_Camera_LightProbeUsage_LightProbeProxyVolume0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        lightProbeUsage: crate::UnityEngine::Rendering::LightProbeUsage,
        lightProbeProxyVolume: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::LightProbeProxyVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            crate::UnityEngine::Rendering::ShadowCastingMode,
                            bool,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rendering::LightProbeUsage,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::LightProbeProxyVolume,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstanced", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        mesh,
                        submeshIndex,
                        material,
                        matrices,
                        count,
                        properties,
                        castShadows,
                        receiveShadows,
                        layer,
                        camera,
                        lightProbeUsage,
                        lightProbeProxyVolume,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced_Mesh_i32_Material_Il2CppArray_i32_MaterialPropertyBlock_ShadowCastingMode__cordl_bool_i32_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            crate::UnityEngine::Rendering::ShadowCastingMode,
                            bool,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstanced", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        mesh,
                        submeshIndex,
                        material,
                        matrices,
                        count,
                        properties,
                        castShadows,
                        receiveShadows,
                        layer,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshNow(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        materialIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            crate::UnityEngine::Matrix4x4,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("DrawMeshNow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshNow", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, matrix, materialIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_ShadowCastingMode_Transform_LightProbeUsage_LightProbeProxyVolume1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        submeshIndex: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        probeAnchor: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        lightProbeUsage: crate::UnityEngine::Rendering::LightProbeUsage,
        lightProbeProxyVolume: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::LightProbeProxyVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            crate::UnityEngine::Rendering::ShadowCastingMode,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            crate::UnityEngine::Rendering::LightProbeUsage,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::LightProbeProxyVolume,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMesh", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        mesh,
                        matrix,
                        material,
                        layer,
                        camera,
                        submeshIndex,
                        properties,
                        castShadows,
                        receiveShadows,
                        probeAnchor,
                        lightProbeUsage,
                        lightProbeProxyVolume,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh__cordl_bool__cordl_bool0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        submeshIndex: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: bool,
        receiveShadows: bool,
        useLightProbes: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            bool,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMesh", 10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        mesh,
                        matrix,
                        material,
                        layer,
                        camera,
                        submeshIndex,
                        properties,
                        castShadows,
                        receiveShadows,
                        useLightProbes,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommandBuffer(
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ExecuteCommandBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteCommandBuffer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_BlitMaterial5(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
        setRT: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Internal_BlitMaterial5")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_BlitMaterial5", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (source, dest, mat, pass, setRT))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        probeAnchor: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        lightProbeUsage: crate::UnityEngine::Rendering::LightProbeUsage,
        lightProbeProxyVolume: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::LightProbeProxyVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            crate::UnityEngine::Rendering::ShadowCastingMode,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            crate::UnityEngine::Rendering::LightProbeUsage,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::LightProbeProxyVolume,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("Internal_DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_DrawMesh", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        mesh,
                        submeshIndex,
                        matrix,
                        material,
                        layer,
                        camera,
                        properties,
                        castShadows,
                        receiveShadows,
                        probeAnchor,
                        lightProbeUsage,
                        lightProbeProxyVolume,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstanced(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        lightProbeUsage: crate::UnityEngine::Rendering::LightProbeUsage,
        lightProbeProxyVolume: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::LightProbeProxyVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            crate::UnityEngine::Rendering::ShadowCastingMode,
                            bool,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rendering::LightProbeUsage,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::LightProbeProxyVolume,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("Internal_DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_DrawMeshInstanced", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        mesh,
                        submeshIndex,
                        material,
                        matrices,
                        count,
                        properties,
                        castShadows,
                        receiveShadows,
                        layer,
                        camera,
                        lightProbeUsage,
                        lightProbeProxyVolume,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshNow2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        subsetIndex: i32,
        matrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            crate::UnityEngine::Matrix4x4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Internal_DrawMeshNow2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_DrawMeshNow2", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, subsetIndex, matrix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshNow2_Injected(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        subsetIndex: i32,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Internal_DrawMeshNow2_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_DrawMeshNow2_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (mesh, subsetIndex, matrix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMesh_Injected(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        probeAnchor: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        lightProbeUsage: crate::UnityEngine::Rendering::LightProbeUsage,
        lightProbeProxyVolume: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::LightProbeProxyVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            crate::UnityEngine::Rendering::ShadowCastingMode,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            crate::UnityEngine::Rendering::LightProbeUsage,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::LightProbeProxyVolume,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("Internal_DrawMesh_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_DrawMesh_Injected", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        mesh,
                        submeshIndex,
                        matrix,
                        material,
                        layer,
                        camera,
                        properties,
                        castShadows,
                        receiveShadows,
                        probeAnchor,
                        lightProbeUsage,
                        lightProbeProxyVolume,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetMaxDrawMeshInstanceCount() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        i32,
                        0usize,
                    >("Internal_GetMaxDrawMeshInstanceCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_GetMaxDrawMeshInstanceCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetNullRT() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("Internal_SetNullRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SetNullRT", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRTSimple(
        color: crate::UnityEngine::RenderBuffer,
        depth: crate::UnityEngine::RenderBuffer,
        mip: i32,
        face: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::RenderBuffer,
                            crate::UnityEngine::RenderBuffer,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Internal_SetRTSimple")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SetRTSimple", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (color, depth, mip, face, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRTSimple_Injected(
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
        depth: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
        mip: i32,
        face: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RenderBuffer,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::RenderBuffer,
                            >,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Internal_SetRTSimple_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SetRTSimple_Injected", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (color, depth, mip, face, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetImpl_RenderBuffer_RenderBuffer_i32_CubemapFace_i32_0(
        colorBuffer: crate::UnityEngine::RenderBuffer,
        depthBuffer: crate::UnityEngine::RenderBuffer,
        mipLevel: i32,
        face: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::RenderBuffer,
                            crate::UnityEngine::RenderBuffer,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetRenderTargetImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTargetImpl", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (colorBuffer, depthBuffer, mipLevel, face, depthSlice),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetImpl_RenderTexture_i32_CubemapFace_i32_1(
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mipLevel: i32,
        face: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetRenderTargetImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTargetImpl", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (rt, mipLevel, face, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTexture1(
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (rt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_i32_CubemapFace2(
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mipLevel: i32,
        face: crate::UnityEngine::CubemapFace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            i32,
                            crate::UnityEngine::CubemapFace,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (rt, mipLevel, face))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_i32_CubemapFace_i32_0(
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mipLevel: i32,
        face: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            i32,
                            crate::UnityEngine::CubemapFace,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (rt, mipLevel, face, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_activeTier() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::GraphicsTier,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::Rendering::GraphicsTier,
                        0usize,
                    >("get_activeTier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_activeTier", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GraphicsTier = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_activeTier(
        value: crate::UnityEngine::Rendering::GraphicsTier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::GraphicsTier),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_activeTier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_activeTier", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Graphics")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Graphics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
