#[cfg(feature = "OVRGLTFLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGLTFLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_jsonData: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
    pub m_glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub m_binaryChunk: crate::GlobalNamespace::OVRBinaryChunk,
    pub m_Nodes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    >,
    pub m_InputAnimationNodes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::OVRGLTFInputNode,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRGLTFAnimatinonNode>,
        >,
    >,
    pub m_AnimationLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::OVRGLTFAnimatinonNode,
                    >,
                >,
            >,
        >,
    >,
    pub m_morphTargetHandlers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler,
            >,
        >,
    >,
    pub m_Shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub m_AlphaBlendShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub m_TextureQuality: crate::GlobalNamespace::OVRTextureQualityFiltering,
    pub m_TextureMipmapBias: f32,
    pub textureUriHandler: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
    >,
}
#[cfg(feature = "OVRGLTFLoader")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRGLTFLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFLoader";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRGLTFLoader")]
impl std::ops::Deref for crate::GlobalNamespace::OVRGLTFLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRGLTFLoader {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFLoader")]
impl crate::GlobalNamespace::OVRGLTFLoader {
    pub fn ApplyTextureQuality(
        qualityLevel: crate::GlobalNamespace::OVRTextureQualityFiltering,
        destTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRTextureQualityFiltering,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ApplyTextureQuality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ApplyTextureQuality", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (qualityLevel, destTexture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnityMaterial(
        &mut self,
        matData: crate::GlobalNamespace::OVRMaterialData,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRMaterialData, bool),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        2usize,
                    >("CreateUnityMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateUnityMaterial", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            method.invoke_unchecked(self, (matData, loadMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DetectTextureQuality(
        srcTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTextureQualityFiltering,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        >),
                        crate::GlobalNamespace::OVRTextureQualityFiltering,
                        1usize,
                    >("DetectTextureQuality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "DetectTextureQuality", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTextureQualityFiltering = unsafe {
            method.invoke_unchecked((), (srcTexture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FlipTraingleIndices(
        indices: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FlipTraingleIndices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FlipTraingleIndices", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (indices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInputNodeType(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRGLTFInputNode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::GlobalNamespace::OVRGLTFInputNode,
                        1usize,
                    >("GetInputNodeType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetInputNodeType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFInputNode = unsafe {
            method.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadGLB(
        &mut self,
        supportAnimation: bool,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRGLTFScene> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool, bool),
                        crate::GlobalNamespace::OVRGLTFScene,
                        2usize,
                    >("LoadGLB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadGLB", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFScene = unsafe {
            method.invoke_unchecked(self, (supportAnimation, loadMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadGLTF(
        &mut self,
        supportAnimation: bool,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(bool, bool), i32, 2usize>("LoadGLTF")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LoadGLTF", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (supportAnimation, loadMips))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ProcessAnimations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ProcessAnimations", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMaterial(
        &mut self,
        matId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMaterialData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32),
                        crate::GlobalNamespace::OVRMaterialData,
                        1usize,
                    >("ProcessMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ProcessMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRMaterialData = unsafe {
            method.invoke_unchecked(self, (matId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMesh(
        &mut self,
        meshNode: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMeshData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                            bool,
                        ),
                        crate::GlobalNamespace::OVRMeshData,
                        2usize,
                    >("ProcessMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ProcessMesh", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRMeshData = unsafe {
            method.invoke_unchecked(self, (meshNode, loadMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        nodeId: i32,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ProcessNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ProcessNode", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, nodeId, loadMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessSkin(
        &mut self,
        skinNode: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::SkinnedMeshRenderer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::SkinnedMeshRenderer,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessSkin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ProcessSkin", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (skinNode, renderer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTexture(
        &mut self,
        textureId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTextureData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32),
                        crate::GlobalNamespace::OVRTextureData,
                        1usize,
                    >("ProcessTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ProcessTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTextureData = unsafe {
            method.invoke_unchecked(self, (textureId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadChunk(
        &mut self,
        glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        _cordl_type: crate::GlobalNamespace::OVRChunkType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            crate::GlobalNamespace::OVRChunkType,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        2usize,
                    >("ReadChunk")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadChunk", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (glbStream, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadMeshAttributes(
        &mut self,
        jsonAttributes: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        totalVertexCount: i32,
        vertexOffset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMeshAttributes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                            i32,
                            i32,
                        ),
                        crate::GlobalNamespace::OVRMeshAttributes,
                        3usize,
                    >("ReadMeshAttributes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadMeshAttributes", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRMeshAttributes = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (jsonAttributes, totalVertexCount, vertexOffset),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMipMapBias(
        &mut self,
        loadedTexturesMipmapBiasing: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetMipMapBias")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetMipMapBias", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (loadedTexturesMipmapBiasing))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetModelAlphaBlendShader(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetModelAlphaBlendShader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetModelAlphaBlendShader", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (shader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetModelShader(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetModelShader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetModelShader", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (shader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTextureQualityFiltering(
        &mut self,
        loadedTexturesQuality: crate::GlobalNamespace::OVRTextureQualityFiltering,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRTextureQualityFiltering),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetTextureQualityFiltering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetTextureQualityFiltering", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (loadedTexturesQuality))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TranscodeTexture(
        &mut self,
        textureData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRTextureData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::GlobalNamespace::OVRTextureData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("TranscodeTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TranscodeTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (textureData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateChunk(
        &mut self,
        glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        _cordl_type: crate::GlobalNamespace::OVRChunkType,
        chunkLength: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            crate::GlobalNamespace::OVRChunkType,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        bool,
                        3usize,
                    >("ValidateChunk")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ValidateChunk", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (glbStream, _cordl_type, chunkLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateGLB(
        &mut self,
        glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                        bool,
                        1usize,
                    >("ValidateGLB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ValidateGLB", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (glbStream))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (fileName))?
        };
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
