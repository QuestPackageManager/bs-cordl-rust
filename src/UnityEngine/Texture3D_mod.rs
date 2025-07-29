#[cfg(feature = "cordl_class_UnityEngine+Texture3D")]
#[repr(C)]
#[derive(Debug)]
pub struct Texture3D {
    __cordl_parent: crate::UnityEngine::Texture,
}
#[cfg(feature = "cordl_class_UnityEngine+Texture3D")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Texture3D {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Texture3D";
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
#[cfg(feature = "UnityEngine+Texture3D")]
impl std::ops::Deref for crate::UnityEngine::Texture3D {
    type Target = crate::UnityEngine::Texture;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Texture3D")]
impl std::ops::DerefMut for crate::UnityEngine::Texture3D {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Texture3D")]
impl crate::UnityEngine::Texture3D {
    pub fn ApplyImpl(
        &mut self,
        updateMipmaps: bool,
        makeNoLongerReadable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ApplyImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyImpl", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (updateMipmaps, makeNoLongerReadable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Apply_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Apply")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Apply",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Apply__cordl_bool__cordl_bool0(
        &mut self,
        updateMipmaps: bool,
        makeNoLongerReadable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Apply")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Apply",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (updateMipmaps, makeNoLongerReadable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Create(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture3D>,
        w: i32,
        h: i32,
        d: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        nativeTex: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture3D>,
                            i32,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::TextureColorSpace,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >("Internal_Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_Create", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (mono, w, h, d, mipCount, format, colorSpace, flags, nativeTex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CreateImpl(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture3D>,
        w: i32,
        h: i32,
        d: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        nativeTex: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture3D>,
                            i32,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::TextureColorSpace,
                            crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                            crate::System::IntPtr,
                        ),
                        bool,
                        9usize,
                    >("Internal_CreateImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_CreateImpl", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (mono, w, h, d, mipCount, format, colorSpace, flags, nativeTex),
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
    pub fn New_GraphicsFormat_TextureCreationFlags2(
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
    pub fn New_GraphicsFormat_TextureCreationFlags_i32_3(
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
    pub fn New_TextureFormat__cordl_bool7(
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
    pub fn New_TextureFormat__cordl_bool_IntPtr9(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        nativeTex: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, depth, textureFormat, mipChain, nativeTex),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool__cordl_bool8(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, depth, textureFormat, mipChain, createUninitialized),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32_4(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, textureFormat, mipCount))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32_IntPtr5(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        nativeTex: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, depth, textureFormat, mipCount, nativeTex),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32_IntPtr__cordl_bool6(
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        nativeTex: crate::System::IntPtr,
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
                    nativeTex,
                    createUninitialized,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetPixels32_Il2CppArray1(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Color32,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetPixels32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetPixels32", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (colors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels32_i32_0(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Color32,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetPixels32")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetPixels32", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (colors, miplevel))?
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
    pub fn _ctor_GraphicsFormat_TextureCreationFlags2(
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
    pub fn _ctor_GraphicsFormat_TextureCreationFlags_i32_3(
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
    pub fn _ctor_TextureFormat__cordl_bool7(
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
    pub fn _ctor_TextureFormat__cordl_bool_IntPtr9(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        nativeTex: crate::System::IntPtr,
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
                            crate::System::IntPtr,
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
                .invoke_unchecked(
                    self,
                    (width, height, depth, textureFormat, mipChain, nativeTex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool8(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        createUninitialized: bool,
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
                    (width, height, depth, textureFormat, mipChain, createUninitialized),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32_4(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, i32, crate::UnityEngine::TextureFormat, i32),
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
                .invoke_unchecked(self, (width, height, depth, textureFormat, mipCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32_IntPtr5(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        nativeTex: crate::System::IntPtr,
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
                            crate::System::IntPtr,
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
                .invoke_unchecked(
                    self,
                    (width, height, depth, textureFormat, mipCount, nativeTex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32_IntPtr__cordl_bool6(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        nativeTex: crate::System::IntPtr,
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
                            crate::System::IntPtr,
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
                        nativeTex,
                        createUninitialized,
                    ),
                )?
        };
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
}
#[cfg(feature = "cordl_class_UnityEngine+Texture3D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Texture3D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
