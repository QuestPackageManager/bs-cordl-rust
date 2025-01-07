#[cfg(feature = "UnityEngine+Texture2DArray")]
#[repr(C)]
#[derive(Debug)]
pub struct Texture2DArray {
    __cordl_parent: crate::UnityEngine::Texture,
}
#[cfg(feature = "UnityEngine+Texture2DArray")]
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
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Texture2DArray")]
impl std::ops::DerefMut for crate::UnityEngine::Texture2DArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_Create",
                (mono, w, h, d, mipCount, format, colorSpace, flags),
            )?;
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
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_CreateImpl",
                (mono, w, h, d, mipCount, format, colorSpace, flags),
            )?;
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
    pub fn New_TextureFormat__cordl_bool8(
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
    pub fn New_TextureFormat__cordl_bool__cordl_bool7(
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
    pub fn New_TextureFormat__cordl_bool__cordl_bool__cordl_bool6(
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
    pub fn New_TextureFormat_i32__cordl_bool5(
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
    pub fn New_TextureFormat_i32__cordl_bool__cordl_bool4(
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
    pub fn ValidateFormat_GraphicsFormat1(
        &mut self,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateFormat", (format, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateFormat_TextureFormat0(
        &mut self,
        format: crate::UnityEngine::TextureFormat,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateFormat", (format, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateIsNotCrunched(
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateIsNotCrunched", (flags))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, format, flags))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, format, flags, mipCount))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, format, flags))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, format, flags, mipCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool8(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, textureFormat, mipChain))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool7(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, textureFormat, mipChain, linear))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool__cordl_bool6(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool5(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, textureFormat, mipCount, linear))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool__cordl_bool4(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
        Ok(__cordl_ret.into())
    }
    pub fn get_allSlices() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_allSlices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isReadable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isReadable", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Texture2DArray")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Texture2DArray {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
