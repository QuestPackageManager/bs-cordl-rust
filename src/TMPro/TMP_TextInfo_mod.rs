#[cfg(feature = "TMPro+TMP_TextInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_TextInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub textComponent: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    pub characterCount: i32,
    pub spriteCount: i32,
    pub spaceCount: i32,
    pub wordCount: i32,
    pub linkCount: i32,
    pub lineCount: i32,
    pub pageCount: i32,
    pub materialCount: i32,
    pub characterInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_CharacterInfo>,
    >,
    pub wordInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_WordInfo>,
    >,
    pub linkInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_LinkInfo>,
    >,
    pub lineInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_LineInfo>,
    >,
    pub pageInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_PageInfo>,
    >,
    pub meshInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_MeshInfo>,
    >,
    pub m_CachedMeshInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_MeshInfo>,
    >,
}
#[cfg(feature = "TMPro+TMP_TextInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_TextInfo => "TMPro"."TMP_TextInfo"
);
#[cfg(feature = "TMPro+TMP_TextInfo")]
impl std::ops::Deref for crate::TMPro::TMP_TextInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_TextInfo")]
impl std::ops::DerefMut for crate::TMPro::TMP_TextInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_TextInfo")]
impl crate::TMPro::TMP_TextInfo {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearAllData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearAllData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearAllMeshInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearAllMeshInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearLineInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLineInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearMeshInfo(
        &mut self,
        updateMesh: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearMeshInfo", (updateMesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearPageInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearPageInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearUnusedVertices(
        &mut self,
        materials: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::TMPro::MaterialReference>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearUnusedVertices", (materials))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyMeshInfoVertexData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_MeshInfo>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::TMPro::TMP_MeshInfo>,
        > = __cordl_object.invoke("CopyMeshInfoVertexData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_TMP_Text2(
        textComponent: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (textComponent))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        characterCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (characterCount))?;
        Ok(__cordl_object.into())
    }
    pub fn ResetVertexLayout(
        &mut self,
        isVolumetric: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetVertexLayout", (isVolumetric))?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize_ByRefMut_i32_0<T>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<T>,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Resize", (array, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize__cordl_bool1<T>(
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<T>,
        >,
        _cordl_size: i32,
        isBlockAllocated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Resize", (array, _cordl_size, isBlockAllocated))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TMP_Text2(
        &mut self,
        textComponent: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (textComponent))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        characterCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (characterCount))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_TextInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_TextInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
