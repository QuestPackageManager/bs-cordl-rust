#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct TextureRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Textures: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::TextureRegistry_TextureInfo,
        >,
    >,
    pub m_TextureToId: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
            crate::UnityEngine::UIElements::TextureId,
        >,
    >,
    pub m_FreeIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            crate::UnityEngine::UIElements::TextureId,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TextureRegistry =>
    "UnityEngine.UIElements"."TextureRegistry"
);
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TextureRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TextureRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
impl crate::UnityEngine::UIElements::TextureRegistry {
    pub const maxTextures: i32 = 2048i32;
    #[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
    pub type TextureInfo = crate::UnityEngine::UIElements::TextureRegistry_TextureInfo;
    pub fn Acquire(
        &mut self,
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextureId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::TextureId = __cordl_object
            .invoke("Acquire", (tex))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocAndAcquire(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dynamic: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextureId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::TextureId = __cordl_object
            .invoke("AllocAndAcquire", (texture, dynamic))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocAndAcquireDynamic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextureId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::TextureId = __cordl_object
            .invoke("AllocAndAcquireDynamic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTexture(
        &mut self,
        id: crate::UnityEngine::UIElements::TextureId,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = __cordl_object
            .invoke("GetTexture", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
        id: crate::UnityEngine::UIElements::TextureId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDynamic(
        &mut self,
        id: crate::UnityEngine::UIElements::TextureId,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDynamic", (id, texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextureRegistry>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TextureRegistry,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TextureRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TextureRegistry_TextureInfo {
    pub texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub dynamic: bool,
    pub refCount: i32,
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::TextureRegistry_TextureInfo => "UnityEngine.UIElements"
    ."TextureRegistry/TextureInfo"
);
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TextureRegistry_TextureInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextureRegistry+TextureInfo")]
impl crate::UnityEngine::UIElements::TextureRegistry_TextureInfo {}
