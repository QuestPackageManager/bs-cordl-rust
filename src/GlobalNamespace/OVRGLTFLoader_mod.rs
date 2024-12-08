#[cfg(feature = "OVRGLTFLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGLTFLoader {
    __cordl_parent: crate::System::Object,
    pub m_jsonData: *mut crate::OVRSimpleJSON::JSONNode,
    pub m_glbStream: *mut crate::System::IO::Stream,
    pub m_binaryChunk: OVRBinaryChunk,
    pub m_Nodes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::GameObject,
    >,
    pub m_InputAnimationNodes: *mut crate::System::Collections::Generic::Dictionary_2<
        OVRGLTFInputNode,
        *mut OVRGLTFAnimatinonNode,
    >,
    pub m_AnimationLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut quest_hook::libil2cpp::Il2CppArray<*mut OVRGLTFAnimatinonNode>,
    >,
    pub m_morphTargetHandlers: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut OVRGLTFAnimationNodeMorphTargetHandler,
    >,
    pub m_Shader: *mut crate::UnityEngine::Shader,
    pub m_AlphaBlendShader: *mut crate::UnityEngine::Shader,
    pub m_TextureQuality: OVRTextureQualityFiltering,
    pub m_TextureMipmapBias: f32,
    pub textureUriHandler: *mut crate::System::Func_3<
        *mut crate::System::String,
        *mut crate::UnityEngine::Material,
        *mut crate::UnityEngine::Texture2D,
    >,
}
#[cfg(feature = "OVRGLTFLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRGLTFLoader => ""."OVRGLTFLoader"
);
#[cfg(feature = "OVRGLTFLoader")]
impl std::ops::Deref for OVRGLTFLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFLoader")]
impl std::ops::DerefMut for OVRGLTFLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFLoader")]
impl OVRGLTFLoader {
    #[cfg(feature = "OVRGLTFLoader+__c__DisplayClass30_0")]
    pub type __c__DisplayClass30_0 = crate::GlobalNamespace::OVRGLTFLoader___c__DisplayClass30_0;
    pub fn CreateUnityMaterial(
        &mut self,
        matData: OVRMaterialData,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Material> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Material = __cordl_object
            .invoke("CreateUnityMaterial", (matData, loadMips))?;
        Ok(__cordl_ret)
    }
    pub fn GetInputNodeType(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<OVRGLTFInputNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: OVRGLTFInputNode = __cordl_object
            .invoke("GetInputNodeType", (name))?;
        Ok(__cordl_ret)
    }
    pub fn LoadGLB(
        &mut self,
        supportAnimation: bool,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<OVRGLTFScene> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: OVRGLTFScene = __cordl_object
            .invoke("LoadGLB", (supportAnimation, loadMips))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray1(
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object)
    }
    pub fn New_String0(
        fileName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fileName))?;
        Ok(__cordl_object)
    }
    pub fn ProcessAnimations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAnimations", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMaterial(
        &mut self,
        matId: i32,
    ) -> quest_hook::libil2cpp::Result<OVRMaterialData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: OVRMaterialData = __cordl_object
            .invoke("ProcessMaterial", (matId))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMesh(
        &mut self,
        meshNode: *mut crate::OVRSimpleJSON::JSONNode,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<OVRMeshData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: OVRMeshData = __cordl_object
            .invoke("ProcessMesh", (meshNode, loadMips))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessNode(
        &mut self,
        node: *mut crate::OVRSimpleJSON::JSONNode,
        nodeId: i32,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNode", (node, nodeId, loadMips))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessSkin(
        &mut self,
        skinNode: *mut crate::OVRSimpleJSON::JSONNode,
        renderer: *mut crate::UnityEngine::SkinnedMeshRenderer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessSkin", (skinNode, renderer))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessTexture(
        &mut self,
        textureId: i32,
    ) -> quest_hook::libil2cpp::Result<OVRTextureData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: OVRTextureData = __cordl_object
            .invoke("ProcessTexture", (textureId))?;
        Ok(__cordl_ret)
    }
    pub fn ReadChunk(
        &mut self,
        glbStream: *mut crate::System::IO::Stream,
        _cordl_type: OVRChunkType,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadChunk", (glbStream, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ReadMeshAttributes(
        &mut self,
        jsonAttributes: *mut crate::OVRSimpleJSON::JSONNode,
        totalVertexCount: i32,
        vertexOffset: i32,
    ) -> quest_hook::libil2cpp::Result<OVRMeshAttributes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: OVRMeshAttributes = __cordl_object
            .invoke(
                "ReadMeshAttributes",
                (jsonAttributes, totalVertexCount, vertexOffset),
            )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetModelAlphaBlendShader(
        &mut self,
        shader: *mut crate::UnityEngine::Shader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetModelAlphaBlendShader", (shader))?;
        Ok(__cordl_ret)
    }
    pub fn SetModelShader(
        &mut self,
        shader: *mut crate::UnityEngine::Shader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetModelShader", (shader))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextureQualityFiltering(
        &mut self,
        loadedTexturesQuality: OVRTextureQualityFiltering,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureQualityFiltering", (loadedTexturesQuality))?;
        Ok(__cordl_ret)
    }
    pub fn TranscodeTexture(
        &mut self,
        textureData: quest_hook::libil2cpp::ByRefMut<OVRTextureData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TranscodeTexture", (textureData))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateChunk(
        &mut self,
        glbStream: *mut crate::System::IO::Stream,
        _cordl_type: OVRChunkType,
        chunkLength: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateChunk", (glbStream, _cordl_type, chunkLength))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateGLB(
        &mut self,
        glbStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidateGLB", (glbStream))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        fileName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fileName))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRGLTFLoader")]
impl quest_hook::libil2cpp::ObjectType for OVRGLTFLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
