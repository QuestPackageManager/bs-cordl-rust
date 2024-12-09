#[cfg(feature = "UnityEngine+Shader")]
#[repr(C)]
#[derive(Debug)]
pub struct Shader {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Shader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Shader => "UnityEngine"."Shader"
);
#[cfg(feature = "UnityEngine+Shader")]
impl std::ops::Deref for crate::UnityEngine::Shader {
    type Target = crate::UnityEngine::Object;
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn FindPropertyIndex(
        &mut self,
        propertyName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FindPropertyIndex", (propertyName))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn FindTextureStack(
        &mut self,
        propertyIndex: i32,
        stackName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        layerIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FindTextureStack", (propertyIndex, stackName, layerIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetDependency(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Shader> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Shader = __cordl_object
            .invoke("GetDependency", (name))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetPropertyAttributes(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPropertyAttributes", (propertyIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPropertyCount", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetPropertyDefaultIntValue(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPropertyDefaultIntValue", (propertyIndex))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetPropertyDescription(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetPropertyDescription", (propertyIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyFlags(
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
        Ok(__cordl_ret)
    }
    pub fn GetPropertyName(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetPropertyName", (propertyIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyNameId(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPropertyNameId", (propertyIndex))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetPropertyTextureDefaultName(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetPropertyTextureDefaultName", (propertyIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyTextureDimension(
        &mut self,
        propertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::TextureDimension> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::TextureDimension = __cordl_object
            .invoke("GetPropertyTextureDimension", (propertyIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyType(
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_disableBatching(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::DisableBatchingType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::DisableBatchingType = __cordl_object
            .invoke("get_disableBatching", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isSupported(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSupported", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_maximumLOD(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maximumLOD", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_passCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_passCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_renderQueue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_renderQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_subshaderCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subshaderCount", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
