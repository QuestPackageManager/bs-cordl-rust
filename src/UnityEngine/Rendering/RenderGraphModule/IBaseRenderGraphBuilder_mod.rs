#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+IBaseRenderGraphBuilder")]
#[derive(Debug)]
#[repr(C)]
pub struct IBaseRenderGraphBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+IBaseRenderGraphBuilder")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "IBaseRenderGraphBuilder";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IBaseRenderGraphBuilder")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IBaseRenderGraphBuilder")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IBaseRenderGraphBuilder")]
impl crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder {
    pub fn AllowGlobalStateModification(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "AllowGlobalStateModification",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AllowGlobalStateModification",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn AllowPassCulling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("AllowPassCulling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AllowPassCulling",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransientBuffer_ByRefMut0(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::BufferDesc,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferDesc,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 1usize>(
                        "CreateTransientBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTransientBuffer",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransientBuffer_ByRefMut1(
        &mut self,
        computebuffer: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 1usize>(
                        "CreateTransientBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTransientBuffer",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (computebuffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransientTexture_ByRefMut0(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 1usize>(
                        "CreateTransientTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTransientTexture",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (desc))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransientTexture_ByRefMut1(
        &mut self,
        texture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 1usize>(
                        "CreateTransientTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateTransientTexture",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableAsyncCompute(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "EnableAsyncCompute",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableAsyncCompute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableFoveatedRasterization(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "EnableFoveatedRasterization",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableFoveatedRasterization",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTextureAfterPass(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        propertyId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalTextureAfterPass"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalTextureAfterPass",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (input, propertyId))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseAllGlobalTextures(
        &mut self,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "UseAllGlobalTextures",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseAllGlobalTextures",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (enable))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseBuffer(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
        >,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
                        >,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 2usize>(
                        "UseBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (input, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseGlobalTexture(
        &mut self,
        propertyId: i32,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                    ), quest_hook::libil2cpp::Void, 2usize>("UseGlobalTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseGlobalTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyId, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseRendererList(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                    >), quest_hook::libil2cpp::Void, 1usize>("UseRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseRendererList",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseTexture(
        &mut self,
        input: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                    ), quest_hook::libil2cpp::Void, 2usize>("UseTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (input, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+IBaseRenderGraphBuilder")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IBaseRenderGraphBuilder")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IBaseRenderGraphBuilder")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
