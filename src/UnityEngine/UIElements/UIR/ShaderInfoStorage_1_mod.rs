#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorage_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderInfoStorage_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::BaseShaderInfoStorage,
    pub m_InitialSize: i32,
    pub m_MaxSize: i32,
    pub m_Format: crate::UnityEngine::TextureFormat,
    pub m_Convert: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<crate::UnityEngine::Color, T>,
    >,
    pub m_Allocator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIRAtlasAllocator,
    >,
    pub m_Texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub m_Texels: crate::Unity::Collections::NativeArray_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorage_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::ShaderInfoStorage_1 < T > =>
    "UnityEngine.UIElements.UIR"."ShaderInfoStorage`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorage_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorage_1<T> {
    type Target = crate::UnityEngine::UIElements::UIR::BaseShaderInfoStorage;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorage_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorage_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorage_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UIR::ShaderInfoStorage_1<T> {
    pub fn AllocateRect(
        &mut self,
        width: i32,
        height: i32,
        uvs: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AllocateRect", (width, height, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn CpuBlit(
        src: crate::Unity::Collections::NativeArray_1<T>,
        srcWidth: i32,
        srcHeight: i32,
        dst: crate::Unity::Collections::NativeArray_1<T>,
        dstWidth: i32,
        dstHeight: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CpuBlit", (src, srcWidth, srcHeight, dst, dstWidth, dstHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateOrExpandTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateOrExpandTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        format: crate::UnityEngine::TextureFormat,
        convert: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<crate::UnityEngine::Color, T>,
        >,
        initialSize: i32,
        maxSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (format, convert, initialSize, maxSize))?;
        Ok(__cordl_object.into())
    }
    pub fn SetTexel(
        &mut self,
        x: i32,
        y: i32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTexel", (x, y, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        format: crate::UnityEngine::TextureFormat,
        convert: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<crate::UnityEngine::Color, T>,
        >,
        initialSize: i32,
        maxSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (format, convert, initialSize, maxSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_texture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = __cordl_object
            .invoke("get_texture", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ShaderInfoStorage_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::ShaderInfoStorage_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
