#[cfg(feature = "UnityEngine+Texture2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Texture2D {
    __cordl_parent: crate::UnityEngine::Texture,
}
#[cfg(feature = "UnityEngine+Texture2D")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Texture2D {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Texture2D";
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
#[cfg(feature = "UnityEngine+Texture2D")]
impl std::ops::Deref for crate::UnityEngine::Texture2D {
    type Target = crate::UnityEngine::Texture;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Texture2D")]
impl std::ops::DerefMut for crate::UnityEngine::Texture2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Texture2D")]
impl crate::UnityEngine::Texture2D {
    pub const streamingMipmapsPriorityMax: i32 = 127i32;
    pub const streamingMipmapsPriorityMin: i32 = -128i32;
    #[cfg(feature = "UnityEngine+Texture2D+EXRFlags")]
    pub type EXRFlags = crate::UnityEngine::Texture2D_EXRFlags;
    pub fn ApplyImpl(
        &mut self,
        updateMipmaps: bool,
        makeNoLongerReadable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ApplyImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ApplyImpl", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (updateMipmaps, makeNoLongerReadable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Apply_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Apply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Apply", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Apply__cordl_bool1(
        &mut self,
        updateMipmaps: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Apply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Apply", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (updateMipmaps))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Apply__cordl_bool__cordl_bool0(
        &mut self,
        updateMipmaps: bool,
        makeNoLongerReadable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool, bool), quest_hook::libil2cpp::Void, 2usize>("Apply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Apply", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (updateMipmaps, makeNoLongerReadable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearMinimumMipmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ClearMinimumMipmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ClearMinimumMipmapLevel", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRequestedMipmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ClearRequestedMipmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ClearRequestedMipmapLevel", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Compress(
        &mut self,
        highQuality: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Compress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Compress", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (highQuality))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateExternalTexture(
        width: i32,
        height: i32,
        format: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
        nativeTex: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::TextureFormat,
                    bool,
                    bool,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                6usize,
            >("CreateExternalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "CreateExternalTexture", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (width, height, format, mipChain, linear, nativeTex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateAtlas(
        sizes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        padding: i32,
        atlasSize: i32,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Rect>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                    >,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Rect,
                        >,
                    >,
                ),
                bool,
                4usize,
            >("GenerateAtlas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GenerateAtlas", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (sizes, padding, atlasSize, results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateAtlasImpl(
        sizes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        padding: i32,
        atlasSize: i32,
        rect: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                    >,
                    i32,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("GenerateAtlasImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GenerateAtlasImpl", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sizes, padding, atlasSize, rect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelBilinearImpl(
        &mut self,
        image: i32,
        mip: i32,
        u: f32,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, f32, f32),
                crate::UnityEngine::Color,
                4usize,
            >("GetPixelBilinearImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixelBilinearImpl", 4usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, (image, mip, u, v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelBilinearImpl_Injected(
        &mut self,
        image: i32,
        mip: i32,
        u: f32,
        v: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    f32,
                    f32,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("GetPixelBilinearImpl_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixelBilinearImpl_Injected", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (image, mip, u, v, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelBilinear_f32_f32_0(
        &mut self,
        u: f32,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, f32),
                crate::UnityEngine::Color,
                2usize,
            >("GetPixelBilinear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixelBilinear", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, (u, v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelBilinear_i32_1(
        &mut self,
        u: f32,
        v: f32,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, f32, i32),
                crate::UnityEngine::Color,
                3usize,
            >("GetPixelBilinear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixelBilinear", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, (u, v, mipLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelData<T>(
        &mut self,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                crate::Unity::Collections::NativeArray_1<T>,
                1usize,
            >("GetPixelData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixelData", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = unsafe {
            method.invoke_unchecked(self, (mipLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelImpl(
        &mut self,
        image: i32,
        mip: i32,
        x: i32,
        y: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32),
                crate::UnityEngine::Color,
                4usize,
            >("GetPixelImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixelImpl", 4usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, (image, mip, x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelImpl_Injected(
        &mut self,
        image: i32,
        mip: i32,
        x: i32,
        y: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("GetPixelImpl_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixelImpl_Injected", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (image, mip, x, y, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixel_i32_1(
        &mut self,
        x: i32,
        y: i32,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                crate::UnityEngine::Color,
                3usize,
            >("GetPixel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixel", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, (x, y, mipLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixel_i32_i32_0(
        &mut self,
        x: i32,
        y: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), crate::UnityEngine::Color, 2usize>("GetPixel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixel", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, (x, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels32_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
                >,
                0usize,
            >("GetPixels32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixels32", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels32_i32_0(
        &mut self,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
                >,
                1usize,
            >("GetPixels32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixels32", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        > = unsafe { method.invoke_unchecked(self, (miplevel))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                >,
                0usize,
            >("GetPixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixels", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels_i32_2(
        &mut self,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                >,
                1usize,
            >("GetPixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixels", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = unsafe { method.invoke_unchecked(self, (miplevel))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels_i32_i32_i32_i32_1(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                >,
                4usize,
            >("GetPixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixels", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = unsafe { method.invoke_unchecked(self, (x, y, blockWidth, blockHeight))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels_i32_i32_i32_i32_i32_0(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                >,
                5usize,
            >("GetPixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetPixels", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = unsafe {
            method.invoke_unchecked(self, (x, y, blockWidth, blockHeight, miplevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRawImageDataSize(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u64, 0usize>("GetRawImageDataSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetRawImageDataSize", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRawTextureData_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("GetRawTextureData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetRawTextureData", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRawTextureData_1<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::Unity::Collections::NativeArray_1<T>,
                0usize,
            >("GetRawTextureData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetRawTextureData", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWritableImageData(
        &mut self,
        frame: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), crate::System::IntPtr, 1usize>("GetWritableImageData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "GetWritableImageData", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked(self, (frame))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Create(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        w: i32,
        h: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        nativeTex: crate::System::IntPtr,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                    i32,
                    i32,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    crate::UnityEngine::TextureColorSpace,
                    crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                9usize,
            >("Internal_Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Internal_Create", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        mono,
                        w,
                        h,
                        mipCount,
                        format,
                        colorSpace,
                        flags,
                        nativeTex,
                        mipmapLimitGroupName,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CreateImpl(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        w: i32,
        h: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        nativeTex: crate::System::IntPtr,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                    i32,
                    i32,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    crate::UnityEngine::TextureColorSpace,
                    crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                bool,
                9usize,
            >("Internal_CreateImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Internal_CreateImpl", 9usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        mono,
                        w,
                        h,
                        mipCount,
                        format,
                        colorSpace,
                        flags,
                        nativeTex,
                        mipmapLimitGroupName,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsRequestedMipmapLevelLoaded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsRequestedMipmapLevelLoaded")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "IsRequestedMipmapLevelLoaded", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRawTextureDataImpl(
        &mut self,
        data: crate::System::IntPtr,
        _cordl_size: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr, u64),
                bool,
                2usize,
            >("LoadRawTextureDataImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "LoadRawTextureDataImpl", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (data, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRawTextureDataImplArray(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                bool,
                1usize,
            >("LoadRawTextureDataImplArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "LoadRawTextureDataImplArray", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (data))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRawTextureData_Il2CppArray1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LoadRawTextureData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "LoadRawTextureData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRawTextureData_IntPtr_i32_0(
        &mut self,
        data: crate::System::IntPtr,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LoadRawTextureData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "LoadRawTextureData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRawTextureData_NativeArray_1_2<T>(
        &mut self,
        data: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Collections::NativeArray_1<T>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LoadRawTextureData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "LoadRawTextureData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_DefaultFormat_TextureCreationFlags1(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, format, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DefaultFormat_i32_Il2CppString_TextureCreationFlags3(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        mipCount: i32,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, format, mipCount, mipmapLimitGroupName, flags),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_DefaultFormat_i32_TextureCreationFlags2(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        mipCount: i32,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, format, mipCount, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_TextureCreationFlags4(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, format, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_TextureCreationFlags_i32_IntPtr_Il2CppString0(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
        nativeTex: crate::System::IntPtr,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, format, flags, mipCount, nativeTex, mipmapLimitGroupName),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_i32_Il2CppString_TextureCreationFlags6(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, format, mipCount, mipmapLimitGroupName, flags),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_i32_TextureCreationFlags5(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, format, mipCount, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool13(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, textureFormat, mipChain))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool__cordl_bool11(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, textureFormat, mipChain, linear))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool__cordl_bool__cordl_bool12(
        width: i32,
        height: i32,
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
                (width, height, textureFormat, mipChain, linear, createUninitialized),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool8(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, textureFormat, mipCount, linear))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool_IntPtr__cordl_bool__cordl_bool_Il2CppString7(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        nativeTex: crate::System::IntPtr,
        createUninitialized: bool,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    width,
                    height,
                    textureFormat,
                    mipCount,
                    linear,
                    nativeTex,
                    createUninitialized,
                    ignoreMipmapLimit,
                    mipmapLimitGroupName,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool__cordl_bool9(
        width: i32,
        height: i32,
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
                (width, height, textureFormat, mipCount, linear, createUninitialized),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool__cordl_bool__cordl_bool_Il2CppString10(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    width,
                    height,
                    textureFormat,
                    mipCount,
                    linear,
                    createUninitialized,
                    ignoreMipmapLimit,
                    mipmapLimitGroupName,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_14(
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height))?;
        Ok(__cordl_object.into())
    }
    pub fn PackTextures_Il2CppArray_i32_2(
        &mut self,
        textures: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
            >,
        >,
        padding: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        >,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
                >,
                2usize,
            >("PackTextures")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "PackTextures", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        > = unsafe { method.invoke_unchecked(self, (textures, padding))? };
        Ok(__cordl_ret.into())
    }
    pub fn PackTextures_i32_1(
        &mut self,
        textures: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
            >,
        >,
        padding: i32,
        maximumAtlasSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        >,
                    >,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
                >,
                3usize,
            >("PackTextures")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "PackTextures", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        > = unsafe {
            method.invoke_unchecked(self, (textures, padding, maximumAtlasSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PackTextures_i32__cordl_bool0(
        &mut self,
        textures: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
            >,
        >,
        padding: i32,
        maximumAtlasSize: i32,
        makeNoLongerReadable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        >,
                    >,
                    i32,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
                >,
                4usize,
            >("PackTextures")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "PackTextures", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (textures, padding, maximumAtlasSize, makeNoLongerReadable),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPixelsImpl(
        &mut self,
        source: crate::UnityEngine::Rect,
        destX: i32,
        destY: i32,
        recalculateMipMaps: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rect, i32, i32, bool),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ReadPixelsImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ReadPixelsImpl", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (source, destX, destY, recalculateMipMaps))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPixelsImpl_Injected(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        destX: i32,
        destY: i32,
        recalculateMipMaps: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                    i32,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ReadPixelsImpl_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ReadPixelsImpl_Injected", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (source, destX, destY, recalculateMipMaps))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPixels_Rect_i32_i32_1(
        &mut self,
        source: crate::UnityEngine::Rect,
        destX: i32,
        destY: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rect, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ReadPixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ReadPixels", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (source, destX, destY))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPixels__cordl_bool0(
        &mut self,
        source: crate::UnityEngine::Rect,
        destX: i32,
        destY: i32,
        recalculateMipMaps: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rect, i32, i32, bool),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ReadPixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ReadPixels", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (source, destX, destY, recalculateMipMaps))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReinitializeImpl(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("ReinitializeImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ReinitializeImpl", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReinitializeWithFormatImpl(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    bool,
                ),
                bool,
                4usize,
            >("ReinitializeWithFormatImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ReinitializeWithFormatImpl", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (width, height, format, hasMipMap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReinitializeWithTextureFormatImpl(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::TextureFormat, bool),
                bool,
                4usize,
            >("ReinitializeWithTextureFormatImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ReinitializeWithTextureFormatImpl", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (width, height, textureFormat, hasMipMap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reinitialize_GraphicsFormat__cordl_bool2(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    bool,
                ),
                bool,
                4usize,
            >("Reinitialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Reinitialize", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (width, height, format, hasMipMap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reinitialize_TextureFormat__cordl_bool1(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::TextureFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::TextureFormat, bool),
                bool,
                4usize,
            >("Reinitialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Reinitialize", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (width, height, format, hasMipMap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reinitialize_i32_i32_0(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("Reinitialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Reinitialize", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Resize_GraphicsFormat__cordl_bool2(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    bool,
                ),
                bool,
                4usize,
            >("Resize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Resize", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (width, height, format, hasMipMap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Resize_TextureFormat__cordl_bool1(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::TextureFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::TextureFormat, bool),
                bool,
                4usize,
            >("Resize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Resize", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (width, height, format, hasMipMap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Resize_i32_i32_0(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("Resize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "Resize", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetAllPixels32(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetAllPixels32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetAllPixels32", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (colors, miplevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBlockOfPixels32(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("SetBlockOfPixels32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetBlockOfPixels32", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (x, y, blockWidth, blockHeight, colors, miplevel),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelDataImpl(
        &mut self,
        data: crate::System::IntPtr,
        mipLevel: i32,
        elementSize: i32,
        dataArraySize: i32,
        sourceDataStartIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr, i32, i32, i32, i32),
                bool,
                5usize,
            >("SetPixelDataImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixelDataImpl", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (data, mipLevel, elementSize, dataArraySize, sourceDataStartIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelDataImplArray(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        mipLevel: i32,
        elementSize: i32,
        dataArraySize: i32,
        sourceDataStartIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Array>, i32, i32, i32, i32),
                bool,
                5usize,
            >("SetPixelDataImplArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixelDataImplArray", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (data, mipLevel, elementSize, dataArraySize, sourceDataStartIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelData_Il2CppArray0<T>(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        mipLevel: i32,
        sourceDataStartIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetPixelData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixelData", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data, mipLevel, sourceDataStartIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelData_NativeArray_1_1<T>(
        &mut self,
        data: crate::Unity::Collections::NativeArray_1<T>,
        mipLevel: i32,
        sourceDataStartIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Collections::NativeArray_1<T>, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetPixelData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixelData", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data, mipLevel, sourceDataStartIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelImpl(
        &mut self,
        image: i32,
        mip: i32,
        x: i32,
        y: i32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, crate::UnityEngine::Color),
                quest_hook::libil2cpp::Void,
                5usize,
            >("SetPixelImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixelImpl", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (image, mip, x, y, color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelImpl_Injected(
        &mut self,
        image: i32,
        mip: i32,
        x: i32,
        y: i32,
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("SetPixelImpl_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixelImpl_Injected", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (image, mip, x, y, color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixel_i32_1(
        &mut self,
        x: i32,
        y: i32,
        color: crate::UnityEngine::Color,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::Color, i32),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetPixel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixel", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (x, y, color, mipLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixel_i32_i32_Color0(
        &mut self,
        x: i32,
        y: i32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::Color),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetPixel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixel", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (x, y, color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels32_Il2CppArray1(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetPixels32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixels32", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (colors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels32_Il2CppArray_i32_0(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetPixels32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixels32", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (colors, miplevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels32_i32_i32_i32_i32_Il2CppArray3(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("SetPixels32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixels32", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (x, y, blockWidth, blockHeight, colors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels32_i32_i32_i32_i32_Il2CppArray_i32_2(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("SetPixels32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixels32", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (x, y, blockWidth, blockHeight, colors, miplevel),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelsImpl(
        &mut self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        pixel: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        miplevel: i32,
        frame: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                    >,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("SetPixelsImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixelsImpl", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (x, y, w, h, pixel, miplevel, frame))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels_Il2CppArray3(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetPixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixels", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (colors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels_Il2CppArray_i32_2(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetPixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixels", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (colors, miplevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels_i32_i32_i32_i32_Il2CppArray1(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("SetPixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixels", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (x, y, blockWidth, blockHeight, colors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels_i32_i32_i32_i32_Il2CppArray_i32_0(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("SetPixels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "SetPixels", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (x, y, blockWidth, blockHeight, colors, miplevel),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateExternalTexture(
        &mut self,
        nativeTex: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UpdateExternalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "UpdateExternalTexture", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nativeTex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateFormat_GraphicsFormat1(
        &mut self,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Experimental::Rendering::GraphicsFormat, i32, i32),
                bool,
                3usize,
            >("ValidateFormat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ValidateFormat", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (format, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateFormat_TextureFormat0(
        &mut self,
        format: crate::UnityEngine::TextureFormat,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::TextureFormat, i32, i32),
                bool,
                3usize,
            >("ValidateFormat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "ValidateFormat", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (format, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_TextureCreationFlags1(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (width, height, format, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_i32_Il2CppString_TextureCreationFlags3(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        mipCount: i32,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::DefaultFormat,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (width, height, format, mipCount, mipmapLimitGroupName, flags),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_i32_TextureCreationFlags2(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        mipCount: i32,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::DefaultFormat,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (width, height, format, mipCount, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_TextureCreationFlags4(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (width, height, format, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_TextureCreationFlags_i32_IntPtr_Il2CppString0(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
        nativeTex: crate::System::IntPtr,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                    i32,
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        width,
                        height,
                        format,
                        flags,
                        mipCount,
                        nativeTex,
                        mipmapLimitGroupName,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_i32_Il2CppString_TextureCreationFlags6(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (width, height, format, mipCount, mipmapLimitGroupName, flags),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_i32_TextureCreationFlags5(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    i32,
                    crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (width, height, format, mipCount, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool13(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::TextureFormat, bool),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (width, height, textureFormat, mipChain))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool11(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::TextureFormat, bool, bool),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (width, height, textureFormat, mipChain, linear),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool__cordl_bool12(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::TextureFormat, bool, bool, bool),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (width, height, textureFormat, mipChain, linear, createUninitialized),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool8(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::TextureFormat, i32, bool),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (width, height, textureFormat, mipCount, linear),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool_IntPtr__cordl_bool__cordl_bool_Il2CppString7(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        nativeTex: crate::System::IntPtr,
        createUninitialized: bool,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::TextureFormat,
                    i32,
                    bool,
                    crate::System::IntPtr,
                    bool,
                    bool,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                9usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        width,
                        height,
                        textureFormat,
                        mipCount,
                        linear,
                        nativeTex,
                        createUninitialized,
                        ignoreMipmapLimit,
                        mipmapLimitGroupName,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool__cordl_bool9(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, crate::UnityEngine::TextureFormat, i32, bool, bool),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (width, height, textureFormat, mipCount, linear, createUninitialized),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool__cordl_bool__cordl_bool_Il2CppString10(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    crate::UnityEngine::TextureFormat,
                    i32,
                    bool,
                    bool,
                    bool,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        width,
                        height,
                        textureFormat,
                        mipCount,
                        linear,
                        createUninitialized,
                        ignoreMipmapLimit,
                        mipmapLimitGroupName,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_14(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_activeMipmapLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_activeMipmapLimit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_activeMipmapLimit", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_blackTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                0usize,
            >("get_blackTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_blackTexture", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_calculatedMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_calculatedMipmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_calculatedMipmapLevel", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_desiredMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_desiredMipmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_desiredMipmapLevel", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextureFormat> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::TextureFormat, 0usize>("get_format")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_format", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::TextureFormat = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_grayTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                0usize,
            >("get_grayTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_grayTexture", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ignoreMipmapLimit(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_ignoreMipmapLimit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_ignoreMipmapLimit", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isPreProcessed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isPreProcessed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_isPreProcessed", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isReadable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isReadable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_isReadable", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_linearGrayTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                0usize,
            >("get_linearGrayTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_linearGrayTexture", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_loadAllMips(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_loadAllMips")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_loadAllMips", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_loadedMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_loadedMipmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_loadedMipmapLevel", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_loadingMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_loadingMipmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_loadingMipmapLevel", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_minimumMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_minimumMipmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_minimumMipmapLevel", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_mipmapLimitGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_mipmapLimitGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_mipmapLimitGroup", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_normalTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                0usize,
            >("get_normalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_normalTexture", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_redTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                0usize,
            >("get_redTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_redTexture", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_requestedMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_requestedMipmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_requestedMipmapLevel", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_streamingMipmaps(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_streamingMipmaps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_streamingMipmaps", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_streamingMipmapsPriority(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_streamingMipmapsPriority")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_streamingMipmapsPriority", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_vtOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_vtOnly")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_vtOnly", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_whiteTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                0usize,
            >("get_whiteTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "get_whiteTexture", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ignoreMipmapLimit(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ignoreMipmapLimit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "set_ignoreMipmapLimit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_loadAllMips(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_loadAllMips")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "set_loadAllMips", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_minimumMipmapLevel(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_minimumMipmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "set_minimumMipmapLevel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_requestedMipmapLevel(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Texture2D as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_requestedMipmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Texture2D as quest_hook::libil2cpp::Type >
                    ::class(), "set_requestedMipmapLevel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Texture2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Texture2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Texture2D+EXRFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Texture2D_EXRFlags {
    #[default]
    CompressPIZ = 8i32,
    CompressRLE = 4i32,
    CompressZIP = 2i32,
    None = 0i32,
    OutputAsFloat = 1i32,
}
#[cfg(feature = "UnityEngine+Texture2D+EXRFlags")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Texture2D_EXRFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Texture2D/EXRFlags";
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
#[cfg(feature = "UnityEngine+Texture2D+EXRFlags")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Texture2D_EXRFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Texture2D+EXRFlags")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Texture2D_EXRFlags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Texture2D+EXRFlags")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Texture2D_EXRFlags {
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
#[cfg(feature = "UnityEngine+Texture2D+EXRFlags")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Texture2D_EXRFlags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
