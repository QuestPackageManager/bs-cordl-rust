#[cfg(feature = "OVRGLTFLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGLTFLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_jsonData: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
    pub m_glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub m_binaryChunk: crate::GlobalNamespace::OVRBinaryChunk,
    pub m_Nodes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::UnityEngine::GameObject>,
    >,
    pub m_InputAnimationNodes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::OVRGLTFInputNode,
            *mut crate::GlobalNamespace::OVRGLTFAnimatinonNode,
        >,
    >,
    pub m_AnimationLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::OVRGLTFAnimatinonNode,
            >,
        >,
    >,
    pub m_morphTargetHandlers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            *mut crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler,
        >,
    >,
    pub m_Shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub m_AlphaBlendShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub m_TextureQuality: crate::GlobalNamespace::OVRTextureQualityFiltering,
    pub m_TextureMipmapBias: f32,
    pub textureUriHandler: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::UnityEngine::Material,
            *mut crate::UnityEngine::Texture2D,
        >,
    >,
}
#[cfg(feature = "OVRGLTFLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRGLTFLoader => ""
    ."OVRGLTFLoader"
);
#[cfg(feature = "OVRGLTFLoader")]
impl std::ops::Deref for crate::GlobalNamespace::OVRGLTFLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRGLTFLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFLoader")]
impl crate::GlobalNamespace::OVRGLTFLoader {
    pub fn ApplyTextureQuality(
        qualityLevel: crate::GlobalNamespace::OVRTextureQualityFiltering,
        destTexture: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyTextureQuality", (qualityLevel, destTexture))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnityMaterial(
        &mut self,
        matData: crate::GlobalNamespace::OVRMaterialData,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("CreateUnityMaterial", (matData, loadMips))?;
        Ok(__cordl_ret.into())
    }
    pub fn DetectTextureQuality(
        srcTexture: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTextureQualityFiltering,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRTextureQualityFiltering = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DetectTextureQuality", (srcTexture))?;
        Ok(__cordl_ret.into())
    }
    pub fn FlipTraingleIndices(
        indices: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FlipTraingleIndices", (indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputNodeType(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRGLTFInputNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFInputNode = __cordl_object
            .invoke("GetInputNodeType", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadGLB(
        &mut self,
        supportAnimation: bool,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRGLTFScene> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFScene = __cordl_object
            .invoke("LoadGLB", (supportAnimation, loadMips))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadGLTF(
        &mut self,
        supportAnimation: bool,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LoadGLTF", (supportAnimation, loadMips))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray1(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString0(
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fileName))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessAnimations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAnimations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMaterial(
        &mut self,
        matId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMaterialData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRMaterialData = __cordl_object
            .invoke("ProcessMaterial", (matId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMesh(
        &mut self,
        meshNode: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMeshData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRMeshData = __cordl_object
            .invoke("ProcessMesh", (meshNode, loadMips))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        nodeId: i32,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNode", (node, nodeId, loadMips))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessSkin(
        &mut self,
        skinNode: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::SkinnedMeshRenderer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessSkin", (skinNode, renderer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTexture(
        &mut self,
        textureId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTextureData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTextureData = __cordl_object
            .invoke("ProcessTexture", (textureId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadChunk(
        &mut self,
        glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        _cordl_type: crate::GlobalNamespace::OVRChunkType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("ReadChunk", (glbStream, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadMeshAttributes(
        &mut self,
        jsonAttributes: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        totalVertexCount: i32,
        vertexOffset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMeshAttributes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRMeshAttributes = __cordl_object
            .invoke(
                "ReadMeshAttributes",
                (jsonAttributes, totalVertexCount, vertexOffset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMipMapBias(
        &mut self,
        loadedTexturesMipmapBiasing: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMipMapBias", (loadedTexturesMipmapBiasing))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetModelAlphaBlendShader(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetModelAlphaBlendShader", (shader))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetModelShader(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetModelShader", (shader))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTextureQualityFiltering(
        &mut self,
        loadedTexturesQuality: crate::GlobalNamespace::OVRTextureQualityFiltering,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureQualityFiltering", (loadedTexturesQuality))?;
        Ok(__cordl_ret.into())
    }
    pub fn TranscodeTexture(
        &mut self,
        textureData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRTextureData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TranscodeTexture", (textureData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateChunk(
        &mut self,
        glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        _cordl_type: crate::GlobalNamespace::OVRChunkType,
        chunkLength: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateChunk", (glbStream, _cordl_type, chunkLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateGLB(
        &mut self,
        glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidateGLB", (glbStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fileName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRGLTFLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRGLTFLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
