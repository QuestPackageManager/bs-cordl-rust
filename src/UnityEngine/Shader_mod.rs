#[cfg(feature = "UnityEngine+Shader")]
#[repr(C)]
#[derive(Debug)]
pub struct Shader {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
}
#[cfg(feature = "UnityEngine+Shader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Shader => "UnityEngine"."Shader"
);
#[cfg(feature = "UnityEngine+Shader")]
impl std::ops::Deref for crate::UnityEngine::Shader {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Shader")]
impl std::ops::DerefMut for crate::UnityEngine::Shader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Shader")]
impl crate::UnityEngine::Shader {
    pub fn CheckPropertyIndex(
        s: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckPropertyIndex", (s, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeywordFast(
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisableKeywordFast", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeywordFast_Injected(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisableKeywordFast_Injected", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeyword_ByRefMut1(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisableKeyword", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeyword_Gc0(
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisableKeyword", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeywordFast(
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableKeywordFast", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeywordFast_Injected(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableKeywordFast_Injected", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeyword_ByRefMut1(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableKeyword", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeyword_Gc0(
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableKeyword", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalFloatArray(
        name: i32,
        values: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractGlobalFloatArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalFloatArrayImpl(
        name: i32,
        val: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractGlobalFloatArrayImpl", (name, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalMatrixArray(
        name: i32,
        values: quest_hook::libil2cpp::Gc<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractGlobalMatrixArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalMatrixArrayImpl(
        name: i32,
        val: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractGlobalMatrixArrayImpl", (name, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalVectorArray(
        name: i32,
        values: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractGlobalVectorArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractGlobalVectorArrayImpl(
        name: i32,
        val: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractGlobalVectorArrayImpl", (name, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn Find(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Find", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindBuiltin(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindBuiltin", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindPassTagValue_ShaderTagId0(
        &mut self,
        passIndex: i32,
        tagName: crate::UnityEngine::Rendering::ShaderTagId,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ShaderTagId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderTagId = __cordl_object
            .invoke("FindPassTagValue", (passIndex, tagName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindPassTagValue_i32_ShaderTagId1(
        &mut self,
        subshaderIndex: i32,
        passIndex: i32,
        tagName: crate::UnityEngine::Rendering::ShaderTagId,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ShaderTagId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderTagId = __cordl_object
            .invoke("FindPassTagValue", (subshaderIndex, passIndex, tagName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindPropertyIndex(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FindPropertyIndex", (propertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindSubshaderTagValue(
        &mut self,
        subshaderIndex: i32,
        tagName: crate::UnityEngine::Rendering::ShaderTagId,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ShaderTagId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderTagId = __cordl_object
            .invoke("FindSubshaderTagValue", (subshaderIndex, tagName))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindTextureStack(
        &mut self,
        propertyIndex: i32,
        stackName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        layerIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FindTextureStack", (propertyIndex, stackName, layerIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindTextureStackImpl(
        s: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIdx: i32,
        stackName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        layerIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindTextureStackImpl", (s, propertyIdx, stackName, layerIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllGlobalKeywords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllGlobalKeywords", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDependency(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader> = __cordl_object
            .invoke("GetDependency", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnabledGlobalKeywords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEnabledGlobalKeywords", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalColor_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalColor", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalColor_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalColor", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArrayCountImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalFloatArrayCountImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArrayImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalFloatArrayImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArray_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalFloatArray", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArray_Gc_Gc2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalFloatArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArray_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalFloatArray", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatArray_i32_Gc3(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalFloatArray", (nameID, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloatImpl(name: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalFloatImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloat_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalFloat", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalFloat_i32_1(nameID: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalFloat", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalIntImpl(name: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalIntImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalInt_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalInt", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalInt_i32_1(nameID: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalInt", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalInteger_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalInteger", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalInteger_i32_1(nameID: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalInteger", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArrayCountImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalMatrixArrayCountImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArrayImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalMatrixArrayImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArray_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalMatrixArray", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArray_Gc_Gc2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalMatrixArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArray_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalMatrixArray", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixArray_i32_Gc3(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalMatrixArray", (nameID, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalMatrixImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrixImpl_Injected(
        name: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalMatrixImpl_Injected", (name, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrix_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalMatrix", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalMatrix_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalMatrix", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalTextureImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalTextureImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalTexture_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalTexture", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalTexture_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalTexture", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArrayCountImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalVectorArrayCountImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArrayImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalVectorArrayImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArray_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalVectorArray", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArray_Gc_Gc2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalVectorArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArray_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalVectorArray", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorArray_i32_Gc3(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalVectorArray", (nameID, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorImpl(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalVectorImpl", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVectorImpl_Injected(
        name: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalVectorImpl_Injected", (name, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVector_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalVector", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalVector_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGlobalVector", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPassCountInSubshader(
        &mut self,
        subshaderIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPassCountInSubshader", (subshaderIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyAttributes_Gc_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyAttributes", (shader, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyAttributes_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("GetPropertyAttributes", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPropertyCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultFloatValue(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetPropertyDefaultFloatValue", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultIntValue_Gc_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyDefaultIntValue", (shader, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultIntValue_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPropertyDefaultIntValue", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultValue(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyDefaultValue", (shader, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultValue_Injected(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyDefaultValue_Injected", (shader, propertyIndex, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDefaultVectorValue(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetPropertyDefaultVectorValue", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDescription_Gc_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyDescription", (shader, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDescription_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPropertyDescription", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyFlags_Gc_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderPropertyFlags,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderPropertyFlags = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyFlags", (shader, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyFlags_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderPropertyFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderPropertyFlags = __cordl_object
            .invoke("GetPropertyFlags", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyNameId_Gc_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyNameId", (shader, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyNameId_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPropertyNameId", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyName_Gc_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyName", (shader, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyName_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPropertyName", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyRangeLimits(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetPropertyRangeLimits", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyTextureDefaultName_Gc_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyTextureDefaultName", (shader, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyTextureDefaultName_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPropertyTextureDefaultName", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyTextureDimension_Gc_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::TextureDimension> {
        let __cordl_ret: crate::UnityEngine::Rendering::TextureDimension = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyTextureDimension", (shader, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyTextureDimension_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::TextureDimension> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::TextureDimension = __cordl_object
            .invoke("GetPropertyTextureDimension", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyType_Gc_i32_0(
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderPropertyType,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderPropertyType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyType", (shader, propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyType_i32_1(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderPropertyType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderPropertyType = __cordl_object
            .invoke("GetPropertyType", (propertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn IDToTag(
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("IDToTag", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_FindPassTagValue(
        &mut self,
        passIndex: i32,
        tagName: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Internal_FindPassTagValue", (passIndex, tagName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_FindPassTagValueInSubShader(
        &mut self,
        subShaderIndex: i32,
        passIndex: i32,
        tagName: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "Internal_FindPassTagValueInSubShader",
                (subShaderIndex, passIndex, tagName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_FindSubshaderTagValue(
        &mut self,
        subShaderIndex: i32,
        tagName: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Internal_FindSubshaderTagValue", (subShaderIndex, tagName))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsKeywordEnabledFast(
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsKeywordEnabledFast", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsKeywordEnabledFast_Injected(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsKeywordEnabledFast_Injected", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsKeywordEnabled_ByRefMut1(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsKeywordEnabled", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsKeywordEnabled_Gc0(
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsKeywordEnabled", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PropertyToID(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PropertyToID", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBufferImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalBufferImpl", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalBuffer", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_Gc2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalBuffer", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_i32_1(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalBuffer", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_i32_3(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalBuffer", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalColor_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalColor", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalColor_i32_1(
        nameID: i32,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalColor", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBufferImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalConstantBufferImpl", (name, value, offset, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalConstantBuffer", (name, value, offset, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_Gc2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalConstantBuffer", (name, value, offset, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_i32_1(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalConstantBuffer", (nameID, value, offset, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_i32_3(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalConstantBuffer", (nameID, value, offset, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantGraphicsBufferImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetGlobalConstantGraphicsBufferImpl",
                (name, value, offset, _cordl_size),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArrayImpl(
        name: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloatArrayImpl", (name, values, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_Gc1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloatArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_Gc3(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloatArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_i32_2(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloatArray", (nameID, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_i32_4(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloatArray", (nameID, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_i32_i32_0(
        name: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloatArray", (name, values, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatImpl(
        name: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloatImpl", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloat", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat_i32_1(
        nameID: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloat", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalGraphicsBufferImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalGraphicsBufferImpl", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalIntImpl(
        name: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalIntImpl", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInt_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalInt", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInt_i32_1(
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalInt", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInteger_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalInteger", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInteger_i32_1(
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalInteger", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArrayImpl(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalMatrixArrayImpl", (name, values, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_Gc1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalMatrixArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_Gc3(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalMatrixArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_i32_2(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalMatrixArray", (nameID, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_i32_4(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalMatrixArray", (nameID, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_i32_i32_0(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalMatrixArray", (name, values, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixImpl(
        name: i32,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalMatrixImpl", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixImpl_Injected(
        name: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalMatrixImpl_Injected", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrix_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalMatrix", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrix_i32_1(
        nameID: i32,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalMatrix", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalRenderTextureImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalRenderTextureImpl", (name, value, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTextureImpl(
        name: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalTextureImpl", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalTexture", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Gc_RenderTextureSubElement2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalTexture", (name, value, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_i32_1(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalTexture", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_i32_RenderTextureSubElement3(
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalTexture", (nameID, value, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArrayImpl(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalVectorArrayImpl", (name, values, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_Gc1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalVectorArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_Gc3(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalVectorArray", (name, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_i32_2(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalVectorArray", (nameID, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_i32_4(
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalVectorArray", (nameID, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_i32_i32_0(
        name: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalVectorArray", (name, values, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorImpl(
        name: i32,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalVectorImpl", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorImpl_Injected(
        name: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalVectorImpl_Injected", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_Gc0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalVector", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_i32_1(
        nameID: i32,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalVector", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetKeyword", (keyword, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKeywordFast(
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetKeywordFast", (keyword, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKeywordFast_Injected(
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GlobalKeyword,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetKeywordFast_Injected", (keyword, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TagToID(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TagToID", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn WarmupAllShaders() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WarmupAllShaders", ())?;
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
    pub fn get_disableBatching(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::DisableBatchingType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::DisableBatchingType = __cordl_object
            .invoke("get_disableBatching", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabledGlobalKeywords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enabledGlobalKeywords", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_globalKeywords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::GlobalKeyword,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_globalKeywords", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_globalMaximumLOD() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_globalMaximumLOD", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_globalRenderPipeline() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_globalRenderPipeline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_globalShaderHardwareTier() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShaderHardwareTier,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::ShaderHardwareTier = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_globalShaderHardwareTier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isSupported(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_keywordSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::LocalKeywordSpace,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::LocalKeywordSpace = __cordl_object
            .invoke("get_keywordSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_keywordSpace_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeywordSpace,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_keywordSpace_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maximumChunksOverride() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_maximumChunksOverride", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maximumLOD(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maximumLOD", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_passCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_passCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderQueue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_renderQueue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subshaderCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subshaderCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_globalMaximumLOD(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_globalMaximumLOD", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_globalRenderPipeline(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_globalRenderPipeline", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_globalShaderHardwareTier(
        value: crate::UnityEngine::Rendering::ShaderHardwareTier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_globalShaderHardwareTier", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maximumChunksOverride(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_maximumChunksOverride", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maximumLOD(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maximumLOD", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Shader")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Shader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
