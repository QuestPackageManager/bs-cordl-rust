#[cfg(feature = "UnityEngine+Sprite")]
#[repr(C)]
#[derive(Debug)]
pub struct Sprite {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Sprite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Sprite => "UnityEngine"."Sprite"
);
#[cfg(feature = "UnityEngine+Sprite")]
impl std::ops::Deref for crate::UnityEngine::Sprite {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Sprite")]
impl std::ops::DerefMut for crate::UnityEngine::Sprite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Sprite")]
impl crate::UnityEngine::Sprite {
    pub fn GetInnerUVs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetInnerUVs", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInnerUVs_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetInnerUVs_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetOuterUVs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetOuterUVs", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetOuterUVs_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOuterUVs_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetPacked(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPacked", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPackingMode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPackingMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPackingRotation(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPackingRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPadding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetPadding", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPadding_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPadding_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetPhysicsShape(
        &mut self,
        shapeIdx: i32,
        physicsShape: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Vector2,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPhysicsShape", (shapeIdx, physicsShape))?;
        Ok(__cordl_ret)
    }
    pub fn GetPhysicsShapeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPhysicsShapeCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPhysicsShapePointCount(
        &mut self,
        shapeIdx: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPhysicsShapePointCount", (shapeIdx))?;
        Ok(__cordl_ret)
    }
    pub fn GetSecondaryTexture(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("GetSecondaryTexture", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetSecondaryTextureCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSecondaryTextureCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSecondaryTextures(
        &mut self,
        secondaryTexture: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::SecondarySpriteTexture,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetSecondaryTextures", (secondaryTexture))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("GetTextureRect", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureRectOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetTextureRectOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureRectOffset_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTextureRectOffset_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureRect_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTextureRect_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn Internal_GetPhysicsShapePointCount(
        &mut self,
        shapeIdx: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Internal_GetPhysicsShapePointCount", (shapeIdx))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OverrideGeometry(
        &mut self,
        vertices: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        triangles: *mut quest_hook::libil2cpp::Il2CppArray<u16>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OverrideGeometry", (vertices, triangles))?;
        Ok(__cordl_ret)
    }
    pub fn OverridePhysicsShape(
        &mut self,
        physicsShapes: *mut crate::System::Collections::Generic::IList_1<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OverridePhysicsShape", (physicsShapes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_associatedAlphaSplitTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("get_associatedAlphaSplitTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_border(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("get_border", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_border_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_border_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_bounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("get_bounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bounds_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_bounds_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_packed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_packed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packingMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpritePackingMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpritePackingMode = __cordl_object
            .invoke("get_packingMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_packingRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpritePackingRotation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpritePackingRotation = __cordl_object
            .invoke("get_packingRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pivot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_pivot", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pivot_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_pivot_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_pixelsPerUnit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pixelsPerUnit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_rect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rect_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_rect_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_spriteAtlasTextureScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_spriteAtlasTextureScale", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_texture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("get_texture", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textureRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_textureRect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_textureRectOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_textureRectOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_triangles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u16>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u16> = __cordl_object
            .invoke("get_triangles", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_uv(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector2,
        > = __cordl_object.invoke("get_uv", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_vertices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector2,
        > = __cordl_object.invoke("get_vertices", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Sprite")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Sprite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}