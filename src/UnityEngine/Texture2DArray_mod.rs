#[cfg(feature = "cordl_class_UnityEngine+Texture2DArray")]
#[repr(C)]
#[derive(Debug)]
pub struct Texture2DArray {
    __cordl_parent: crate::UnityEngine::Texture,
}
#[cfg(feature = "cordl_class_UnityEngine+Texture2DArray")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Texture2DArray {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Texture2DArray";
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
#[cfg(feature = "UnityEngine+Texture2DArray")]
impl std::ops::Deref for crate::UnityEngine::Texture2DArray {
    type Target = crate::UnityEngine::Texture;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Texture2DArray")]
impl std::ops::DerefMut for crate::UnityEngine::Texture2DArray {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Texture2DArray")]
impl crate::UnityEngine::Texture2DArray {
    pub fn Internal_Create(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2DArray>,
        w: i32,
        h: i32,
        d: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Texture2DArray,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::TextureColorSpace,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >("Internal_Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_Create", 10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        mono,
                        w,
                        h,
                        d,
                        mipCount,
                        format,
                        colorSpace,
                        flags,
                        ignoreMipmapLimit,
                        mipmapLimitGroupName,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CreateImpl(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2DArray>,
        w: i32,
        h: i32,
        d: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Texture2DArray,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::TextureColorSpace,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        10usize,
                    >("Internal_CreateImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_CreateImpl", 10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        mono,
                        w,
                        h,
                        d,
                        mipCount,
                        format,
                        colorSpace,
                        flags,
                        ignoreMipmapLimit,
                        mipmapLimitGroupName,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CreateImpl_Injected(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2DArray>,
        w: i32,
        h: i32,
        d: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Texture2DArray,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::TextureColorSpace,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Bindings::ManagedSpanWrapper,
                            >,
                        ),
                        bool,
                        10usize,
                    >("Internal_CreateImpl_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_CreateImpl_Injected", 10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        mono,
                        w,
                        h,
                        d,
                        mipCount,
                        format,
                        colorSpace,
                        flags,
                        ignoreMipmapLimit,
                        mipmapLimitGroupName,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_DefaultFormat_TextureCreationFlags0(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, format, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DefaultFormat_TextureCreationFlags_i32_1(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, format, flags, mipCount))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DefaultFormat_TextureCreationFlags_i32_MipmapLimitDescriptor2(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
        mipmapLimitDescriptor: crate::UnityEngine::MipmapLimitDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, depth, format, flags, mipCount, mipmapLimitDescriptor),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_TextureCreationFlags3(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, format, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_TextureCreationFlags_i32_4(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, format, flags, mipCount))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_TextureCreationFlags_i32_MipmapLimitDescriptor5(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
        mipmapLimitDescriptor: crate::UnityEngine::MipmapLimitDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, depth, format, flags, mipCount, mipmapLimitDescriptor),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool11(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, textureFormat, mipChain))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool__cordl_bool10(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, depth, textureFormat, mipChain, linear),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool__cordl_bool__cordl_bool9(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    width,
                    height,
                    depth,
                    textureFormat,
                    mipChain,
                    linear,
                    createUninitialized,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool8(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, depth, textureFormat, mipCount, linear),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool__cordl_bool7(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    width,
                    height,
                    depth,
                    textureFormat,
                    mipCount,
                    linear,
                    createUninitialized,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool__cordl_bool_MipmapLimitDescriptor6(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
        mipmapLimitDescriptor: crate::UnityEngine::MipmapLimitDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    width,
                    height,
                    depth,
                    textureFormat,
                    mipCount,
                    linear,
                    createUninitialized,
                    mipmapLimitDescriptor,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ValidateFormat_GraphicsFormat1(
        &mut self,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            i32,
                            i32,
                        ),
                        bool,
                        3usize,
                    >("ValidateFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateFormat", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (format, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateFormat_TextureFormat0(
        &mut self,
        format: crate::UnityEngine::TextureFormat,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::TextureFormat, i32, i32),
                        bool,
                        3usize,
                    >("ValidateFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateFormat", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (format, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateIsNotCrunched(
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::TextureCreationFlags),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ValidateIsNotCrunched")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateIsNotCrunched", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_TextureCreationFlags0(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::DefaultFormat,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (width, height, depth, format, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_TextureCreationFlags_i32_1(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::DefaultFormat,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (width, height, depth, format, flags, mipCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_TextureCreationFlags_i32_MipmapLimitDescriptor2(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
        mipmapLimitDescriptor: crate::UnityEngine::MipmapLimitDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::DefaultFormat,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            i32,
                            crate::UnityEngine::MipmapLimitDescriptor,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        width,
                        height,
                        depth,
                        format,
                        flags,
                        mipCount,
                        mipmapLimitDescriptor,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_TextureCreationFlags3(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (width, height, depth, format, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_TextureCreationFlags_i32_4(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (width, height, depth, format, flags, mipCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_TextureCreationFlags_i32_MipmapLimitDescriptor5(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
        mipmapLimitDescriptor: crate::UnityEngine::MipmapLimitDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            i32,
                            crate::UnityEngine::MipmapLimitDescriptor,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        width,
                        height,
                        depth,
                        format,
                        flags,
                        mipCount,
                        mipmapLimitDescriptor,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool11(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, i32, crate::UnityEngine::TextureFormat, bool),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (width, height, depth, textureFormat, mipChain))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool10(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, i32, crate::UnityEngine::TextureFormat, bool, bool),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (width, height, depth, textureFormat, mipChain, linear),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool__cordl_bool9(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::TextureFormat,
                            bool,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        width,
                        height,
                        depth,
                        textureFormat,
                        mipChain,
                        linear,
                        createUninitialized,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool8(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, i32, crate::UnityEngine::TextureFormat, i32, bool),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (width, height, depth, textureFormat, mipCount, linear),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool__cordl_bool7(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::TextureFormat,
                            i32,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        width,
                        height,
                        depth,
                        textureFormat,
                        mipCount,
                        linear,
                        createUninitialized,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool__cordl_bool_MipmapLimitDescriptor6(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
        mipmapLimitDescriptor: crate::UnityEngine::MipmapLimitDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::TextureFormat,
                            i32,
                            bool,
                            bool,
                            crate::UnityEngine::MipmapLimitDescriptor,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        width,
                        height,
                        depth,
                        textureFormat,
                        mipCount,
                        linear,
                        createUninitialized,
                        mipmapLimitDescriptor,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_allSlices() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_allSlices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_allSlices", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isReadable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isReadable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_isReadable", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isReadable_Injected(
        _unity_self: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        bool,
                        1usize,
                    >("get_isReadable_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_isReadable_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Texture2DArray")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Texture2DArray {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
