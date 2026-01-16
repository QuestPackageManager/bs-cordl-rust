#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+IRasterRenderGraphBuilder")]
#[derive(Debug)]
#[repr(C)]
pub struct IRasterRenderGraphBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+IRasterRenderGraphBuilder")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule";
    const CLASS_NAME: &'static str = "IRasterRenderGraphBuilder";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IRasterRenderGraphBuilder")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IRasterRenderGraphBuilder")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IRasterRenderGraphBuilder")]
impl crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder {
    pub fn SetInputAttachment_TextureHandle_i32_AccessFlags0(
        &mut self,
        tex: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        index: i32,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetInputAttachment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInputAttachment",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (tex, index, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInputAttachment_i32_i32_1(
        &mut self,
        tex: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        index: i32,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
        mipLevel: i32,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetInputAttachment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInputAttachment",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (tex, index, flags, mipLevel, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRandomAccessAttachment(
        &mut self,
        tex: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        index: i32,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 3usize>(
                        "SetRandomAccessAttachment",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRandomAccessAttachment",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (tex, index, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderAttachmentDepth_TextureHandle_AccessFlags0(
        &mut self,
        tex: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetRenderAttachmentDepth"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderAttachmentDepth",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (tex, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderAttachmentDepth_i32_i32_1(
        &mut self,
        tex: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
        mipLevel: i32,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetRenderAttachmentDepth"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderAttachmentDepth",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (tex, flags, mipLevel, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderAttachment_TextureHandle_i32_AccessFlags0(
        &mut self,
        tex: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        index: i32,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRenderAttachment"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderAttachment",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (tex, index, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderAttachment_i32_i32_1(
        &mut self,
        tex: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        index: i32,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
        mipLevel: i32,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetRenderAttachment"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderAttachment",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (tex, index, flags, mipLevel, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderFunc<PassData>(
        &mut self,
        renderFunc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::BaseRenderFunc_2<
                PassData,
                crate::UnityEngine::Rendering::RenderGraphModule::RasterGraphContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        PassData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::BaseRenderFunc_2<
                            PassData,
                            crate::UnityEngine::Rendering::RenderGraphModule::RasterGraphContext,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("SetRenderFunc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderFunc",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderFunc))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseBufferRandomAccess_AccessFlags0(
        &mut self,
        tex: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
        index: i32,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 3usize>(
                        "UseBufferRandomAccess",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseBufferRandomAccess",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (tex, index, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseBufferRandomAccess__cordl_bool_AccessFlags1(
        &mut self,
        tex: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
        index: i32,
        preserveCounterValue: bool,
        flags: crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
                        i32,
                        bool,
                        crate::UnityEngine::Rendering::RenderGraphModule::AccessFlags,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle, 4usize>(
                        "UseBufferRandomAccess",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseBufferRandomAccess",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (tex, index, preserveCounterValue, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+IRasterRenderGraphBuilder")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IRasterRenderGraphBuilder")]
impl AsRef<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder
{
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IRasterRenderGraphBuilder")]
impl AsMut<crate::System::IDisposable>
    for crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder
{
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IRasterRenderGraphBuilder")]
impl AsRef<crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder>
    for crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+IRasterRenderGraphBuilder")]
impl AsMut<crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder>
    for crate::UnityEngine::Rendering::RenderGraphModule::IRasterRenderGraphBuilder
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Rendering::RenderGraphModule::IBaseRenderGraphBuilder {
        unsafe { std::mem::transmute(self) }
    }
}
