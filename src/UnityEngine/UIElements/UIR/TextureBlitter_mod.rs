#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
#[repr(C)]
#[derive(Debug)]
pub struct TextureBlitter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_SingleBlit: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo,
        >,
    >,
    pub m_BlitMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_Properties: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MaterialPropertyBlock,
    >,
    pub m_Viewport: crate::UnityEngine::RectInt,
    pub m_PrevRT: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    pub m_PendingBlits: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo,
        >,
    >,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::TextureBlitter =>
    "UnityEngine.UIElements.UIR"."TextureBlitter"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl crate::UnityEngine::UIElements::UIR::TextureBlitter {
    #[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
    pub type BlitInfo = crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo;
    pub fn BeginBlit(
        &mut self,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginBlit", (dst))?;
        Ok(__cordl_ret.into())
    }
    pub fn BlitOneNow(
        &mut self,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        srcRect: crate::UnityEngine::RectInt,
        dstPos: crate::UnityEngine::Vector2Int,
        addBorder: bool,
        tint: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BlitOneNow", (dst, src, srcRect, dstPos, addBorder, tint))?;
        Ok(__cordl_ret.into())
    }
    pub fn Commit(
        &mut self,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Commit", (dst))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoBlit(
        &mut self,
        blitInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo,
            >,
        >,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoBlit", (blitInfos, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndBlit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndBlit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (capacity))?;
        Ok(__cordl_object.into())
    }
    pub fn QueueBlit(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        srcRect: crate::UnityEngine::RectInt,
        dstPos: crate::UnityEngine::Vector2Int,
        addBorder: bool,
        tint: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueBlit", (src, srcRect, dstPos, addBorder, tint))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (capacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposed", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::TextureBlitter {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextureBlitter_BlitInfo {
    pub src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub srcRect: crate::UnityEngine::RectInt,
    pub dstPos: crate::UnityEngine::Vector2Int,
    pub border: i32,
    pub tint: crate::UnityEngine::Color,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo =>
    "UnityEngine.UIElements.UIR"."TextureBlitter/BlitInfo"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+TextureBlitter+BlitInfo")]
impl crate::UnityEngine::UIElements::UIR::TextureBlitter_BlitInfo {}
