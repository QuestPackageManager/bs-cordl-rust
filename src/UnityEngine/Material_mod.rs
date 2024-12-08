#[cfg(feature = "UnityEngine+Material")]
#[repr(C)]
#[derive(Debug)]
pub struct Material {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Material")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Material => "UnityEngine"
    ."Material"
);
#[cfg(feature = "UnityEngine+Material")]
impl std::ops::Deref for crate::UnityEngine::Material {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Material")]
impl std::ops::DerefMut for crate::UnityEngine::Material {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Material")]
impl crate::UnityEngine::Material {
    pub fn ComputeCRC(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ComputeCRC", ())?;
        Ok(__cordl_ret)
    }
    pub fn CopyMatchingPropertiesFromMaterial(
        &mut self,
        mat: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyMatchingPropertiesFromMaterial", (mat))?;
        Ok(__cordl_ret)
    }
    pub fn CopyPropertiesFromMaterial(
        &mut self,
        mat: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyPropertiesFromMaterial", (mat))?;
        Ok(__cordl_ret)
    }
    pub fn DisableKeyword_ByRefMut1(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableKeyword", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn DisableKeyword_String0(
        &mut self,
        keyword: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableKeyword", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn DisableLocalKeyword(
        &mut self,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableLocalKeyword", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn DisableLocalKeyword_Injected(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableLocalKeyword_Injected", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn EnableKeyword_ByRefMut1(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableKeyword", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn EnableKeyword_String0(
        &mut self,
        keyword: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableKeyword", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn EnableLocalKeyword(
        &mut self,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableLocalKeyword", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn EnableLocalKeyword_Injected(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableLocalKeyword_Injected", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn ExtractColorArray(
        &mut self,
        name: i32,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Color,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtractColorArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn ExtractColorArrayImpl(
        &mut self,
        name: i32,
        val: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtractColorArrayImpl", (name, val))?;
        Ok(__cordl_ret)
    }
    pub fn ExtractFloatArray(
        &mut self,
        name: i32,
        values: *mut crate::System::Collections::Generic::List_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtractFloatArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn ExtractFloatArrayImpl(
        &mut self,
        name: i32,
        val: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtractFloatArrayImpl", (name, val))?;
        Ok(__cordl_ret)
    }
    pub fn ExtractMatrixArray(
        &mut self,
        name: i32,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Matrix4x4,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtractMatrixArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn ExtractMatrixArrayImpl(
        &mut self,
        name: i32,
        val: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtractMatrixArrayImpl", (name, val))?;
        Ok(__cordl_ret)
    }
    pub fn ExtractVectorArray(
        &mut self,
        name: i32,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Vector4,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtractVectorArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn ExtractVectorArrayImpl(
        &mut self,
        name: i32,
        val: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExtractVectorArrayImpl", (name, val))?;
        Ok(__cordl_ret)
    }
    pub fn FindPass(
        &mut self,
        passName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindPass", (passName))?;
        Ok(__cordl_ret)
    }
    pub fn GetBuffer(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::GraphicsBufferHandle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::GraphicsBufferHandle = __cordl_object
            .invoke("GetBuffer", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetBufferImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::GraphicsBufferHandle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::GraphicsBufferHandle = __cordl_object
            .invoke("GetBufferImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetBufferImpl_Injected(
        &mut self,
        name: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::GraphicsBufferHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBufferImpl_Injected", (name, ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorArrayCountImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetColorArrayCountImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorArrayImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Color,
        > = __cordl_object.invoke("GetColorArrayImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorArray_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Color,
        > = __cordl_object.invoke("GetColorArray", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorArray_String_List_1_2(
        &mut self,
        name: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Color,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetColorArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorArray_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Color,
        > = __cordl_object.invoke("GetColorArray", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorArray_i32_List_1_3(
        &mut self,
        nameID: i32,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Color,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetColorArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetColorImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorImpl_Injected(
        &mut self,
        name: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetColorImpl_Injected", (name, ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetColor_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetColor", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetColor_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetColor", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetConstantBuffer(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::GraphicsBufferHandle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::GraphicsBufferHandle = __cordl_object
            .invoke("GetConstantBuffer", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetConstantBufferImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::GraphicsBufferHandle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::GraphicsBufferHandle = __cordl_object
            .invoke("GetConstantBufferImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetConstantBufferImpl_Injected(
        &mut self,
        name: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::GraphicsBufferHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetConstantBufferImpl_Injected", (name, ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnabledKeywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::LocalKeyword,
        > = __cordl_object.invoke("GetEnabledKeywords", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFirstPropertyNameIdByAttribute(
        &mut self,
        attributeFlag: crate::UnityEngine::Rendering::ShaderPropertyFlags,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetFirstPropertyNameIdByAttribute", (attributeFlag))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloatArrayCountImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetFloatArrayCountImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloatArrayImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<f32> = __cordl_object
            .invoke("GetFloatArrayImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloatArray_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<f32> = __cordl_object
            .invoke("GetFloatArray", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloatArray_String_List_1_2(
        &mut self,
        name: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::List_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetFloatArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloatArray_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<f32> = __cordl_object
            .invoke("GetFloatArray", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloatArray_i32_List_1_3(
        &mut self,
        nameID: i32,
        values: *mut crate::System::Collections::Generic::List_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetFloatArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloatImpl(&mut self, name: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFloatImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloat_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFloat", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetFloat_i32_1(&mut self, nameID: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetFloat", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetIntImpl(&mut self, name: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIntImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetInt_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInt", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetInt_i32_1(&mut self, nameID: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInt", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetInteger_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInteger", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetInteger_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInteger", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatrixArrayCountImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMatrixArrayCountImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatrixArrayImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Matrix4x4,
        > = __cordl_object.invoke("GetMatrixArrayImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatrixArray_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Matrix4x4,
        > = __cordl_object.invoke("GetMatrixArray", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatrixArray_String_List_1_2(
        &mut self,
        name: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Matrix4x4,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetMatrixArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatrixArray_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Matrix4x4,
        > = __cordl_object.invoke("GetMatrixArray", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatrixArray_i32_List_1_3(
        &mut self,
        nameID: i32,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Matrix4x4,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetMatrixArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatrixImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("GetMatrixImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatrixImpl_Injected(
        &mut self,
        name: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetMatrixImpl_Injected", (name, ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatrix_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("GetMatrix", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetMatrix_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("GetMatrix", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetPassName(
        &mut self,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetPassName", (pass))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyNames(
        &mut self,
        _cordl_type: crate::UnityEngine::MaterialPropertyType,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetPropertyNames", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyNamesImpl(
        &mut self,
        propertyType: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetPropertyNamesImpl", (propertyType))?;
        Ok(__cordl_ret)
    }
    pub fn GetShaderKeywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetShaderKeywords", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetShaderPassEnabled(
        &mut self,
        passName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetShaderPassEnabled", (passName))?;
        Ok(__cordl_ret)
    }
    pub fn GetTagImpl(
        &mut self,
        tag: *mut crate::System::String,
        currentSubShaderOnly: bool,
        defaultValue: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTagImpl", (tag, currentSubShaderOnly, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn GetTag_String0(
        &mut self,
        tag: *mut crate::System::String,
        searchFallbacks: bool,
        defaultValue: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTag", (tag, searchFallbacks, defaultValue))?;
        Ok(__cordl_ret)
    }
    pub fn GetTag_String__cordl_bool1(
        &mut self,
        tag: *mut crate::System::String,
        searchFallbacks: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTag", (tag, searchFallbacks))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture = __cordl_object
            .invoke("GetTextureImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureOffset_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetTextureOffset", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureOffset_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetTextureOffset", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetTexturePropertyNameIDsInternal(
        &mut self,
        outNames: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTexturePropertyNameIDsInternal", (outNames))?;
        Ok(__cordl_ret)
    }
    pub fn GetTexturePropertyNameIDs_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("GetTexturePropertyNameIDs", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTexturePropertyNameIDs_List_1_1(
        &mut self,
        outNames: *mut crate::System::Collections::Generic::List_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTexturePropertyNameIDs", (outNames))?;
        Ok(__cordl_ret)
    }
    pub fn GetTexturePropertyNamesInternal(
        &mut self,
        outNames: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTexturePropertyNamesInternal", (outNames))?;
        Ok(__cordl_ret)
    }
    pub fn GetTexturePropertyNames_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetTexturePropertyNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTexturePropertyNames_List_1_1(
        &mut self,
        outNames: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTexturePropertyNames", (outNames))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureScaleAndOffsetImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetTextureScaleAndOffsetImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureScaleAndOffsetImpl_Injected(
        &mut self,
        name: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTextureScaleAndOffsetImpl_Injected", (name, ret))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureScale_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetTextureScale", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextureScale_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetTextureScale", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetTexture_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture = __cordl_object
            .invoke("GetTexture", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetTexture_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture = __cordl_object
            .invoke("GetTexture", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetVectorArrayCountImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetVectorArrayCountImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetVectorArrayImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector4,
        > = __cordl_object.invoke("GetVectorArrayImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetVectorArray_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector4,
        > = __cordl_object.invoke("GetVectorArray", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetVectorArray_String_List_1_2(
        &mut self,
        name: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Vector4,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetVectorArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn GetVectorArray_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Vector4,
        > = __cordl_object.invoke("GetVectorArray", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn GetVectorArray_i32_List_1_3(
        &mut self,
        nameID: i32,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Vector4,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetVectorArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn GetVector_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetVector", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetVector_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("GetVector", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn HasBufferImpl(&mut self, name: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasBufferImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasBuffer_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasBuffer", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasBuffer_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasBuffer", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn HasColor_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasColor", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasColor_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasColor", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn HasConstantBufferImpl(
        &mut self,
        name: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasConstantBufferImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasConstantBuffer_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasConstantBuffer", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasConstantBuffer_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasConstantBuffer", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn HasFloatImpl(&mut self, name: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFloatImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasFloat_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFloat", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasFloat_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFloat", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn HasIntImpl(&mut self, name: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasIntImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasInt_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasInt", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasInt_i32_1(&mut self, nameID: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasInt", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn HasInteger_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasInteger", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasInteger_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasInteger", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn HasMatrixImpl(&mut self, name: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasMatrixImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasMatrix_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasMatrix", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasMatrix_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasMatrix", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn HasProperty_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasProperty", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasProperty_i32_0(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasProperty", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn HasTextureImpl(&mut self, name: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasTextureImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasTexture_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasTexture", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasTexture_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasTexture", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn HasVectorImpl(&mut self, name: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasVectorImpl", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasVector_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasVector", (name))?;
        Ok(__cordl_ret)
    }
    pub fn HasVector_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasVector", (nameID))?;
        Ok(__cordl_ret)
    }
    pub fn IsKeywordEnabled_ByRefMut1(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsKeywordEnabled", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn IsKeywordEnabled_String0(
        &mut self,
        keyword: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsKeywordEnabled", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn IsLocalKeywordEnabled(
        &mut self,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsLocalKeywordEnabled", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn IsLocalKeywordEnabled_Injected(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsLocalKeywordEnabled_Injected", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn Lerp(
        &mut self,
        start: *mut crate::UnityEngine::Material,
        end: *mut crate::UnityEngine::Material,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Lerp", (start, end, t))?;
        Ok(__cordl_ret)
    }
    pub fn New_Material1(
        source: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source))?;
        Ok(__cordl_object)
    }
    pub fn New_Shader0(
        shader: *mut crate::UnityEngine::Shader,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (shader))?;
        Ok(__cordl_object)
    }
    pub fn New_String2(
        contents: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contents))?;
        Ok(__cordl_object)
    }
    pub fn SetBufferImpl(
        &mut self,
        name: i32,
        value: *mut crate::UnityEngine::ComputeBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBufferImpl", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetBuffer_String_ComputeBuffer0(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::UnityEngine::ComputeBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBuffer", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetBuffer_String_GraphicsBuffer2(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::UnityEngine::GraphicsBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBuffer", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetBuffer_i32_ComputeBuffer1(
        &mut self,
        nameID: i32,
        value: *mut crate::UnityEngine::ComputeBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBuffer", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetBuffer_i32_GraphicsBuffer3(
        &mut self,
        nameID: i32,
        value: *mut crate::UnityEngine::GraphicsBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBuffer", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorArrayImpl(
        &mut self,
        name: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorArrayImpl", (name, values, count))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorArray_String_Il2CppArray3(
        &mut self,
        name: *mut crate::System::String,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorArray_String_List_1_1(
        &mut self,
        name: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Color,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorArray_i32_Il2CppArray4(
        &mut self,
        nameID: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorArray_i32_Il2CppArray_i32_0(
        &mut self,
        name: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorArray", (name, values, count))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorArray_i32_List_1_2(
        &mut self,
        nameID: i32,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Color,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorImpl(
        &mut self,
        name: i32,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorImpl", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorImpl_Injected(
        &mut self,
        name: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorImpl_Injected", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetColor_String0(
        &mut self,
        name: *mut crate::System::String,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetColor_i32_1(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetConstantBufferImpl(
        &mut self,
        name: i32,
        value: *mut crate::UnityEngine::ComputeBuffer,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetConstantBufferImpl", (name, value, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn SetConstantBuffer_String_ComputeBuffer0(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::UnityEngine::ComputeBuffer,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetConstantBuffer", (name, value, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn SetConstantBuffer_String_GraphicsBuffer2(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::UnityEngine::GraphicsBuffer,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetConstantBuffer", (name, value, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn SetConstantBuffer_i32_ComputeBuffer1(
        &mut self,
        nameID: i32,
        value: *mut crate::UnityEngine::ComputeBuffer,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetConstantBuffer", (nameID, value, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn SetConstantBuffer_i32_GraphicsBuffer3(
        &mut self,
        nameID: i32,
        value: *mut crate::UnityEngine::GraphicsBuffer,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetConstantBuffer", (nameID, value, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn SetConstantGraphicsBufferImpl(
        &mut self,
        name: i32,
        value: *mut crate::UnityEngine::GraphicsBuffer,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetConstantGraphicsBufferImpl",
                (name, value, offset, _cordl_size),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetEnabledKeywords(
        &mut self,
        keywords: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEnabledKeywords", (keywords))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatArrayImpl(
        &mut self,
        name: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatArrayImpl", (name, values, count))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatArray_String_Il2CppArray3(
        &mut self,
        name: *mut crate::System::String,
        values: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatArray_String_List_1_1(
        &mut self,
        name: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::List_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatArray_i32_Il2CppArray4(
        &mut self,
        nameID: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatArray_i32_Il2CppArray_i32_0(
        &mut self,
        name: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatArray", (name, values, count))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatArray_i32_List_1_2(
        &mut self,
        nameID: i32,
        values: *mut crate::System::Collections::Generic::List_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloatImpl(
        &mut self,
        name: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloatImpl", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloat_String0(
        &mut self,
        name: *mut crate::System::String,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloat", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetFloat_i32_1(
        &mut self,
        nameID: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFloat", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetGraphicsBufferImpl(
        &mut self,
        name: i32,
        value: *mut crate::UnityEngine::GraphicsBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGraphicsBufferImpl", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetIntImpl(
        &mut self,
        name: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIntImpl", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetInt_String0(
        &mut self,
        name: *mut crate::System::String,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInt", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetInt_i32_1(
        &mut self,
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInt", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetInteger_String0(
        &mut self,
        name: *mut crate::System::String,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInteger", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetInteger_i32_1(
        &mut self,
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInteger", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetKeyword(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKeyword", (keyword, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetLocalKeyword(
        &mut self,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalKeyword", (keyword, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetLocalKeyword_Injected(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalKeyword_Injected", (keyword, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetMatrixArrayImpl(
        &mut self,
        name: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMatrixArrayImpl", (name, values, count))?;
        Ok(__cordl_ret)
    }
    pub fn SetMatrixArray_String_Il2CppArray3(
        &mut self,
        name: *mut crate::System::String,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMatrixArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetMatrixArray_String_List_1_1(
        &mut self,
        name: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Matrix4x4,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMatrixArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetMatrixArray_i32_Il2CppArray4(
        &mut self,
        nameID: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMatrixArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetMatrixArray_i32_Il2CppArray_i32_0(
        &mut self,
        name: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMatrixArray", (name, values, count))?;
        Ok(__cordl_ret)
    }
    pub fn SetMatrixArray_i32_List_1_2(
        &mut self,
        nameID: i32,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Matrix4x4,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMatrixArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetMatrixImpl(
        &mut self,
        name: i32,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMatrixImpl", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetMatrixImpl_Injected(
        &mut self,
        name: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMatrixImpl_Injected", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetMatrix_String0(
        &mut self,
        name: *mut crate::System::String,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMatrix", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetMatrix_i32_1(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMatrix", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetOverrideTag(
        &mut self,
        tag: *mut crate::System::String,
        val: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOverrideTag", (tag, val))?;
        Ok(__cordl_ret)
    }
    pub fn SetPass(&mut self, pass: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SetPass", (pass))?;
        Ok(__cordl_ret)
    }
    pub fn SetRenderTextureImpl(
        &mut self,
        name: i32,
        value: *mut crate::UnityEngine::RenderTexture,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRenderTextureImpl", (name, value, element))?;
        Ok(__cordl_ret)
    }
    pub fn SetShaderKeywords(
        &mut self,
        names: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetShaderKeywords", (names))?;
        Ok(__cordl_ret)
    }
    pub fn SetShaderPassEnabled(
        &mut self,
        passName: *mut crate::System::String,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetShaderPassEnabled", (passName, enabled))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextureImpl(
        &mut self,
        name: i32,
        value: *mut crate::UnityEngine::Texture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureImpl", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextureOffsetImpl(
        &mut self,
        name: i32,
        offset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureOffsetImpl", (name, offset))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextureOffsetImpl_Injected(
        &mut self,
        name: i32,
        offset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureOffsetImpl_Injected", (name, offset))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextureOffset_String0(
        &mut self,
        name: *mut crate::System::String,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureOffset", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextureOffset_i32_1(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureOffset", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextureScaleImpl(
        &mut self,
        name: i32,
        scale: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureScaleImpl", (name, scale))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextureScaleImpl_Injected(
        &mut self,
        name: i32,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureScaleImpl_Injected", (name, scale))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextureScale_String0(
        &mut self,
        name: *mut crate::System::String,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureScale", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextureScale_i32_1(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureScale", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetTexture_String_RenderTexture_RenderTextureSubElement2(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::UnityEngine::RenderTexture,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTexture", (name, value, element))?;
        Ok(__cordl_ret)
    }
    pub fn SetTexture_String_Texture0(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::UnityEngine::Texture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTexture", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetTexture_i32_RenderTexture_RenderTextureSubElement3(
        &mut self,
        nameID: i32,
        value: *mut crate::UnityEngine::RenderTexture,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTexture", (nameID, value, element))?;
        Ok(__cordl_ret)
    }
    pub fn SetTexture_i32_Texture1(
        &mut self,
        nameID: i32,
        value: *mut crate::UnityEngine::Texture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTexture", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetVectorArrayImpl(
        &mut self,
        name: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVectorArrayImpl", (name, values, count))?;
        Ok(__cordl_ret)
    }
    pub fn SetVectorArray_String_Il2CppArray3(
        &mut self,
        name: *mut crate::System::String,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVectorArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetVectorArray_String_List_1_1(
        &mut self,
        name: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Vector4,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVectorArray", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetVectorArray_i32_Il2CppArray4(
        &mut self,
        nameID: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVectorArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetVectorArray_i32_Il2CppArray_i32_0(
        &mut self,
        name: i32,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVectorArray", (name, values, count))?;
        Ok(__cordl_ret)
    }
    pub fn SetVectorArray_i32_List_1_2(
        &mut self,
        nameID: i32,
        values: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Vector4,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVectorArray", (nameID, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetVector_String0(
        &mut self,
        name: *mut crate::System::String,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVector", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetVector_i32_1(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVector", (nameID, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Material1(
        &mut self,
        source: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Shader0(
        &mut self,
        shader: *mut crate::UnityEngine::Shader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (shader))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String2(
        &mut self,
        contents: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contents))?;
        Ok(__cordl_ret)
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_doubleSidedGI(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_doubleSidedGI", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enableInstancing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableInstancing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enabledKeywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::LocalKeyword,
        > = __cordl_object.invoke("get_enabledKeywords", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_globalIlluminationFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::MaterialGlobalIlluminationFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::MaterialGlobalIlluminationFlags = __cordl_object
            .invoke("get_globalIlluminationFlags", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mainTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture = __cordl_object
            .invoke("get_mainTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mainTextureOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_mainTextureOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mainTextureScale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_mainTextureScale", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_passCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_passCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rawRenderQueue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_rawRenderQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_renderQueue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_renderQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_shader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("get_shader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_shaderKeywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_shaderKeywords", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_doubleSidedGI(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_doubleSidedGI", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_enableInstancing(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableInstancing", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_enabledKeywords(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enabledKeywords", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_globalIlluminationFlags(
        &mut self,
        value: crate::UnityEngine::MaterialGlobalIlluminationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_globalIlluminationFlags", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_mainTexture(
        &mut self,
        value: *mut crate::UnityEngine::Texture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mainTexture", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_mainTextureOffset(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mainTextureOffset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_mainTextureScale(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mainTextureScale", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_renderQueue(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_renderQueue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_shader(
        &mut self,
        value: *mut crate::UnityEngine::Shader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_shader", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_shaderKeywords(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_shaderKeywords", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Material")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Material {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
