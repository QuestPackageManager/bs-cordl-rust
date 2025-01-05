#[cfg(feature = "UnityEngine+SpriteRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteRenderer {
    __cordl_parent: crate::UnityEngine::Renderer,
    pub m_SpriteChangeEvent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Events::UnityEvent_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::SpriteRenderer>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+SpriteRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpriteRenderer => "UnityEngine"
    ."SpriteRenderer"
);
#[cfg(feature = "UnityEngine+SpriteRenderer")]
impl std::ops::Deref for crate::UnityEngine::SpriteRenderer {
    type Target = crate::UnityEngine::Renderer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpriteRenderer")]
impl std::ops::DerefMut for crate::UnityEngine::SpriteRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpriteRenderer")]
impl crate::UnityEngine::SpriteRenderer {
    pub fn GetCurrentMeshData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Mesh_MeshDataArray> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Mesh_MeshDataArray = __cordl_object
            .invoke("GetCurrentMeshData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentMeshDataPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetCurrentMeshDataPtr", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSecondaryTextureProperties(
        &mut self,
        mbp: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetSecondaryTextureProperties", (mbp))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("GetSpriteBounds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetSpriteBounds(
        &mut self,
        mode: crate::UnityEngine::SpriteDrawMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("Internal_GetSpriteBounds", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetSpriteBounds_Injected(
        &mut self,
        mode: crate::UnityEngine::SpriteDrawMode,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Internal_GetSpriteBounds_Injected", (mode, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSpriteChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSpriteChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterSpriteChangeCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::SpriteRenderer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterSpriteChangeCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterSpriteChangeCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::SpriteRenderer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterSpriteChangeCallback", (callback))?;
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
    pub fn get_adaptiveModeThreshold(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_adaptiveModeThreshold", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_color_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_drawMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpriteDrawMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpriteDrawMode = __cordl_object
            .invoke("get_drawMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flipX(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_flipX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flipY(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_flipY", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maskInteraction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpriteMaskInteraction> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpriteMaskInteraction = __cordl_object
            .invoke("get_maskInteraction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shouldSupportTiling(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_shouldSupportTiling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_size(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_size", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_size_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_size_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sprite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = __cordl_object
            .invoke("get_sprite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spriteSortPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpriteSortPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpriteSortPoint = __cordl_object
            .invoke("get_spriteSortPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tileMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpriteTileMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpriteTileMode = __cordl_object
            .invoke("get_tileMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_adaptiveModeThreshold(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_adaptiveModeThreshold", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_drawMode(
        &mut self,
        value: crate::UnityEngine::SpriteDrawMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_drawMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flipX(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flipX", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flipY(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flipY", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maskInteraction(
        &mut self,
        value: crate::UnityEngine::SpriteMaskInteraction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maskInteraction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_size(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_size", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_size_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_size_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sprite(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sprite", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_spriteSortPoint(
        &mut self,
        value: crate::UnityEngine::SpriteSortPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spriteSortPoint", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tileMode(
        &mut self,
        value: crate::UnityEngine::SpriteTileMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tileMode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SpriteRenderer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SpriteRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
