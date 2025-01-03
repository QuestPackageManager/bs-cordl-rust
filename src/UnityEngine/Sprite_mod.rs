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
    pub fn CreateSprite(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
        border: crate::UnityEngine::Vector4,
        generateFallbackPhysicsShape: bool,
        secondaryTexture: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::SecondarySpriteTexture,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateSprite",
                (
                    texture,
                    rect,
                    pivot,
                    pixelsPerUnit,
                    extrude,
                    meshType,
                    border,
                    generateFallbackPhysicsShape,
                    secondaryTexture,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpriteWithoutTextureScripting(
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsToUnits: f32,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateSpriteWithoutTextureScripting",
                (rect, pivot, pixelsToUnits, texture),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpriteWithoutTextureScripting_Injected(
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        pivot: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        pixelsToUnits: f32,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateSpriteWithoutTextureScripting_Injected",
                (rect, pivot, pixelsToUnits, texture),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSprite_Injected(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        pivot: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
        border: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        generateFallbackPhysicsShape: bool,
        secondaryTexture: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::SecondarySpriteTexture,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateSprite_Injected",
                (
                    texture,
                    rect,
                    pivot,
                    pixelsPerUnit,
                    extrude,
                    meshType,
                    border,
                    generateFallbackPhysicsShape,
                    secondaryTexture,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Rect_Vector2_f32_1(
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsToUnits: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (rect, pivot, pixelsToUnits))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Rect_Vector2_f32_Texture2D0(
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsToUnits: f32,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (rect, pivot, pixelsToUnits, texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_8(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (texture, rect, pivot))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_7(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (texture, rect, pivot, pixelsPerUnit))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_u32_6(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (texture, rect, pivot, pixelsPerUnit, extrude))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_u32_SpriteMeshType5(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (texture, rect, pivot, pixelsPerUnit, extrude, meshType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_u32_SpriteMeshType_Vector4_4(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
        border: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (texture, rect, pivot, pixelsPerUnit, extrude, meshType, border),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_u32_SpriteMeshType_Vector4__cordl_bool2(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
        border: crate::UnityEngine::Vector4,
        generateFallbackPhysicsShape: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (
                    texture,
                    rect,
                    pivot,
                    pixelsPerUnit,
                    extrude,
                    meshType,
                    border,
                    generateFallbackPhysicsShape,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Texture2D_Rect_Vector2_f32_u32_SpriteMeshType_Vector4__cordl_bool_Il2CppArray3(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        rect: crate::UnityEngine::Rect,
        pivot: crate::UnityEngine::Vector2,
        pixelsPerUnit: f32,
        extrude: u32,
        meshType: crate::UnityEngine::SpriteMeshType,
        border: crate::UnityEngine::Vector4,
        generateFallbackPhysicsShape: bool,
        secondaryTextures: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::SecondarySpriteTexture,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (
                    texture,
                    rect,
                    pivot,
                    pixelsPerUnit,
                    extrude,
                    meshType,
                    border,
                    generateFallbackPhysicsShape,
                    secondaryTextures,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInnerUVs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetInnerUVs", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetOuterUVs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetOuterUVs", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetPacked(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPacked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPackingMode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPackingMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPackingRotation(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPackingRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPadding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetPadding", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsShape(
        &mut self,
        shapeIdx: i32,
        physicsShape: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPhysicsShape", (shapeIdx, physicsShape))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsShapeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPhysicsShapeCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPhysicsShapeImpl(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        shapeIdx: i32,
        physicsShape: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPhysicsShapeImpl", (sprite, shapeIdx, physicsShape))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetSecondaryTexture(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = __cordl_object
            .invoke("GetSecondaryTexture", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSecondaryTextureCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSecondaryTextureCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSecondaryTextures(
        &mut self,
        secondaryTexture: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::SecondarySpriteTexture,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetSecondaryTextures", (secondaryTexture))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("GetTextureRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureRectOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetTextureRectOffset", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OverrideGeometry(
        &mut self,
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OverrideGeometry", (vertices, triangles))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverridePhysicsShapeCount(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        physicsShapeCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverridePhysicsShapeCount", (sprite, physicsShapeCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverridePhysicsShape_IList_1_0(
        &mut self,
        physicsShapes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OverridePhysicsShape", (physicsShapes))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverridePhysicsShape_Sprite_Il2CppArray_i32_1(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        physicsShape: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverridePhysicsShape", (sprite, physicsShape, idx))?;
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
    pub fn get_associatedAlphaSplitTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = __cordl_object
            .invoke("get_associatedAlphaSplitTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_border(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("get_border", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_bounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("get_bounds", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_packed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_packed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packingMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpritePackingMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpritePackingMode = __cordl_object
            .invoke("get_packingMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packingRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpritePackingRotation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpritePackingRotation = __cordl_object
            .invoke("get_packingRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pivot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_pivot", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelsPerUnit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pixelsPerUnit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_rect", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_spriteAtlasTextureScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_spriteAtlasTextureScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_texture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = __cordl_object
            .invoke("get_texture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textureRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_textureRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textureRectOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_textureRectOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_triangles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u16>,
        > = __cordl_object.invoke("get_triangles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = __cordl_object.invoke("get_uv", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = __cordl_object.invoke("get_vertices", ())?;
        Ok(__cordl_ret.into())
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
