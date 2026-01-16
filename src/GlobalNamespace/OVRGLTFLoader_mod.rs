#[cfg(feature = "cordl_class_OVRGLTFLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGLTFLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_deferredStream: quest_hook::libil2cpp::Gc<
        crate::System::Func_1<quest_hook::libil2cpp::Gc<crate::System::IO::Stream>>,
    >,
    pub m_jsonData: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
    pub m_glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub m_Nodes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
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
    pub scene: crate::GlobalNamespace::OVRGLTFScene,
    pub textureUriHandler: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
    >,
    pub m_textures: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
    >,
    pub m_materials: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
    >,
    pub m_processingNodesStart: f32,
    pub _dataAccessor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRGLTFAccessor,
    >,
}
#[cfg(feature = "cordl_class_OVRGLTFLoader")]
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
    pub const LoadingMaxTimePerFrame: f32 = 0.014285714f32;
    pub fn ApplyTextureQuality(
        qualityLevel: crate::GlobalNamespace::OVRTextureQualityFiltering,
        destTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ApplyTextureQuality", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (qualityLevel, destTexture))?
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
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRMaterialData, bool),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        2usize,
                    >("CreateUnityMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateUnityMaterial", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            cordl_method_info.invoke_unchecked(self, (matData, loadMips))?
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
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DetectTextureQuality", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTextureQualityFiltering = unsafe {
            cordl_method_info.invoke_unchecked((), (srcTexture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FlipTriangleIndices(
        indices: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FlipTriangleIndices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FlipTriangleIndices", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (indices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInputNodeType(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRGLTFInputNode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::GlobalNamespace::OVRGLTFInputNode,
                        1usize,
                    >("GetInputNodeType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInputNodeType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFInputNode = unsafe {
            cordl_method_info.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeGLBLoad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                        0usize,
                    >("InitializeGLBLoad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeGLBLoad", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadGLB(
        &mut self,
        supportAnimation: bool,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRGLTFScene> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool),
                        crate::GlobalNamespace::OVRGLTFScene,
                        2usize,
                    >("LoadGLB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "LoadGLB",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFScene = unsafe {
            cordl_method_info.invoke_unchecked(self, (supportAnimation, loadMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadGLBCoroutine(
        &mut self,
        supportAnimation: bool,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        2usize,
                    >("LoadGLBCoroutine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadGLBCoroutine", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (supportAnimation, loadMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadGLTF(
        &mut self,
        supportAnimation: bool,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        2usize,
                    >("LoadGLTF")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadGLTF", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (supportAnimation, loadMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_Func_1_2(
        deferredStream: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<quest_hook::libil2cpp::Gc<crate::System::IO::Stream>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (deferredStream))?;
        Ok(__cordl_object.into())
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        0usize,
                    >("ProcessAnimations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessAnimations", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMaterial(
        &mut self,
        matId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMaterialData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::GlobalNamespace::OVRMaterialData,
                        1usize,
                    >("ProcessMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRMaterialData = unsafe {
            cordl_method_info.invoke_unchecked(self, (matId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMesh(
        &mut self,
        meshNode: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        loadMips: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMeshData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessMesh", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRMeshData = unsafe {
            cordl_method_info.invoke_unchecked(self, (meshNode, loadMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessNode(
        &mut self,
        nodes: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONArray>,
        nodeId: i32,
        loadMips: bool,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONArray>,
                            i32,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        4usize,
                    >("ProcessNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessNode", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (nodes, nodeId, loadMips, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessSkin(
        &mut self,
        skinNode: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::SkinnedMeshRenderer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessSkin", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (skinNode, renderer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTexture(
        &mut self,
        textureId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTextureData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::GlobalNamespace::OVRTextureData,
                        1usize,
                    >("ProcessTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTextureData = unsafe {
            cordl_method_info.invoke_unchecked(self, (textureId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadChunk(
        glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        _cordl_type: crate::GlobalNamespace::OVRChunkType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadChunk", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (glbStream, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadMeshAttributes(
        &mut self,
        jsonAttributes: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        totalVertexCount: i32,
        vertexOffset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRMeshAttributes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadMeshAttributes", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRMeshAttributes = unsafe {
            cordl_method_info
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
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetMipMapBias")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetMipMapBias", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (loadedTexturesMipmapBiasing))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetModelAlphaBlendShader(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetModelAlphaBlendShader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetModelAlphaBlendShader", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (shader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetModelShader(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetModelShader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetModelShader", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (shader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTextureQualityFiltering(
        &mut self,
        loadedTexturesQuality: crate::GlobalNamespace::OVRTextureQualityFiltering,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRTextureQualityFiltering),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetTextureQualityFiltering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetTextureQualityFiltering", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (loadedTexturesQuality))?
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
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TranscodeTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (textureData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateChunk(
        glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        _cordl_type: crate::GlobalNamespace::OVRChunkType,
        chunkLength: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateChunk", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (glbStream, _cordl_type, chunkLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateGLB(
        glbStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                        bool,
                        1usize,
                    >("ValidateGLB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateGLB", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (glbStream))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _LoadGLBCoroutine_b__26_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                        0usize,
                    >("<LoadGLBCoroutine>b__26_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<LoadGLBCoroutine>b__26_0", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Func_1_2(
        &mut self,
        deferredStream: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<quest_hook::libil2cpp::Gc<crate::System::IO::Stream>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Func_1<
                                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (deferredStream))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (fileName))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRGLTFLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRGLTFLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
