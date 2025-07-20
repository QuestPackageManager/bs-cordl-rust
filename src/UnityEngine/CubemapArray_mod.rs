#[cfg(feature = "UnityEngine+CubemapArray")]
#[repr(C)]
#[derive(Debug)]
pub struct CubemapArray {
    __cordl_parent: crate::UnityEngine::Texture,
}
#[cfg(feature = "UnityEngine+CubemapArray")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::CubemapArray {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "CubemapArray";
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
#[cfg(feature = "UnityEngine+CubemapArray")]
impl std::ops::Deref for crate::UnityEngine::CubemapArray {
    type Target = crate::UnityEngine::Texture;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CubemapArray")]
impl std::ops::DerefMut for crate::UnityEngine::CubemapArray {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CubemapArray")]
impl crate::UnityEngine::CubemapArray {
    pub fn Internal_Create(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::CubemapArray>,
        ext: i32,
        count: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::CubemapArray>,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::TextureColorSpace,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("Internal_Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Internal_Create", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (mono, ext, count, mipCount, format, colorSpace, flags),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CreateImpl(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::CubemapArray>,
        ext: i32,
        count: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::CubemapArray>,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::TextureColorSpace,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                        ),
                        bool,
                        7usize,
                    >("Internal_CreateImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Internal_CreateImpl", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (mono, ext, count, mipCount, format, colorSpace, flags),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_DefaultFormat_TextureCreationFlags0(
        width: i32,
        cubemapCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, cubemapCount, format, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DefaultFormat_TextureCreationFlags_i32_1(
        width: i32,
        cubemapCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, cubemapCount, format, flags, mipCount))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_TextureCreationFlags2(
        width: i32,
        cubemapCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, cubemapCount, format, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_TextureCreationFlags_i32_3(
        width: i32,
        cubemapCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, cubemapCount, format, flags, mipCount))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool8(
        width: i32,
        cubemapCount: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, cubemapCount, textureFormat, mipChain))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool__cordl_bool7(
        width: i32,
        cubemapCount: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, cubemapCount, textureFormat, mipChain, linear),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool__cordl_bool__cordl_bool6(
        width: i32,
        cubemapCount: i32,
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
                    cubemapCount,
                    textureFormat,
                    mipChain,
                    linear,
                    createUninitialized,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool5(
        width: i32,
        cubemapCount: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, cubemapCount, textureFormat, mipCount, linear),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool__cordl_bool4(
        width: i32,
        cubemapCount: i32,
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
                    cubemapCount,
                    textureFormat,
                    mipCount,
                    linear,
                    createUninitialized,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ValidateIsNotCrunched(
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::TextureCreationFlags),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ValidateIsNotCrunched")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ValidateIsNotCrunched", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_TextureCreationFlags0(
        &mut self,
        width: i32,
        cubemapCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::DefaultFormat,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (width, cubemapCount, format, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_TextureCreationFlags_i32_1(
        &mut self,
        width: i32,
        cubemapCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::DefaultFormat,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (width, cubemapCount, format, flags, mipCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_TextureCreationFlags2(
        &mut self,
        width: i32,
        cubemapCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (width, cubemapCount, format, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_TextureCreationFlags_i32_3(
        &mut self,
        width: i32,
        cubemapCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (width, cubemapCount, format, flags, mipCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool8(
        &mut self,
        width: i32,
        cubemapCount: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32, crate::UnityEngine::TextureFormat, bool),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (width, cubemapCount, textureFormat, mipChain))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool7(
        &mut self,
        width: i32,
        cubemapCount: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32, crate::UnityEngine::TextureFormat, bool, bool),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (width, cubemapCount, textureFormat, mipChain, linear),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool__cordl_bool6(
        &mut self,
        width: i32,
        cubemapCount: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32, crate::UnityEngine::TextureFormat, bool, bool, bool),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        width,
                        cubemapCount,
                        textureFormat,
                        mipChain,
                        linear,
                        createUninitialized,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool5(
        &mut self,
        width: i32,
        cubemapCount: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32, crate::UnityEngine::TextureFormat, i32, bool),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (width, cubemapCount, textureFormat, mipCount, linear),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool__cordl_bool4(
        &mut self,
        width: i32,
        cubemapCount: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, i32, crate::UnityEngine::TextureFormat, i32, bool, bool),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        width,
                        cubemapCount,
                        textureFormat,
                        mipCount,
                        linear,
                        createUninitialized,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isReadable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), bool, 0usize>("get_isReadable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_isReadable", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+CubemapArray")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::CubemapArray {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
