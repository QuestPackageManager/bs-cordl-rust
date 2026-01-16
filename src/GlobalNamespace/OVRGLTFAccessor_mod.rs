#[cfg(feature = "cordl_class_OVRGLTFAccessor")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGLTFAccessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _accessors: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::OVRGLTFAccessor_GLTFAccessor,
        >,
    >,
    pub _bufferViews: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::OVRGLTFAccessor_GLTFBufferView,
        >,
    >,
    pub _buffers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::OVRGLTFAccessor_GLTFBuffer,
        >,
    >,
    pub _binaryChunk: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub _binaryChunkLength: i32,
    pub _binaryChunkStart: i32,
    pub _reader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
    pub _activeGltfAccessor: crate::GlobalNamespace::OVRGLTFAccessor_GLTFAccessor,
    pub _activeBufferView: crate::GlobalNamespace::OVRGLTFAccessor_GLTFBufferView,
    pub _activeBuffer: crate::GlobalNamespace::OVRGLTFAccessor_GLTFBuffer,
    pub _activeBufferOffset: i32,
    pub _requireStrideSeek: bool,
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRGLTFAccessor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFAccessor";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "OVRGLTFAccessor")]
impl std::ops::Deref for crate::GlobalNamespace::OVRGLTFAccessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAccessor")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRGLTFAccessor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAccessor")]
impl crate::GlobalNamespace::OVRGLTFAccessor {
    #[cfg(feature = "OVRGLTFAccessor+GLTFAccessor")]
    pub type GLTFAccessor = crate::GlobalNamespace::OVRGLTFAccessor_GLTFAccessor;
    #[cfg(feature = "OVRGLTFAccessor+GLTFBuffer")]
    pub type GLTFBuffer = crate::GlobalNamespace::OVRGLTFAccessor_GLTFBuffer;
    #[cfg(feature = "OVRGLTFAccessor+GLTFBufferView")]
    pub type GLTFBufferView = crate::GlobalNamespace::OVRGLTFAccessor_GLTFBufferView;
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDataCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetDataCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDataCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxValueForType(
        &mut self,
        _cordl_type: crate::GlobalNamespace::OVRGLTFComponentType,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::GlobalNamespace::OVRGLTFComponentType), f32, 1usize>(
                        "GetMaxValueForType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMaxValueForType",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStrideForType(
        &mut self,
        _cordl_type: crate::GlobalNamespace::OVRGLTFComponentType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::GlobalNamespace::OVRGLTFComponentType), i32, 1usize>(
                        "GetStrideForType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetStrideForType",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        accessorsRoot: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        bufferViewsRoot: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        buffersRoot: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        binaryChunkReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
        binaryChinkStart: i32,
        binaryChunkLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                accessorsRoot,
                bufferViewsRoot,
                buffersRoot,
                binaryChunkReader,
                binaryChinkStart,
                binaryChunkLength,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn ReadAsFloat(
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
        _cordl_type: crate::GlobalNamespace::OVRGLTFComponentType,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
                        crate::GlobalNamespace::OVRGLTFComponentType,
                    ), f32, 2usize>("ReadAsFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadAsFloat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked((), (reader, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsInt(
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
        _cordl_type: crate::GlobalNamespace::OVRGLTFComponentType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
                        crate::GlobalNamespace::OVRGLTFComponentType,
                    ), i32, 2usize>("ReadAsInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadAsInt",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (reader, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadBuffer(
        &mut self,
        bufferViewIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        1usize,
                    >("ReadBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadBuffer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>> =
            unsafe { cordl_method_info.invoke_unchecked(self, (bufferViewIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                    >, 0usize>("ReadColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadColor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFloat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<f32>,
                        >,
                        0usize,
                    >("ReadFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadFloat", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadInt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<i32>,
                        >,
                        0usize,
                    >("ReadInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ReadInt",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadJoints(
        &mut self,
        resultsBoneWeights: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("ReadJoints")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadJoints",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (resultsBoneWeights))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadMatrix4x4(
        &mut self,
        conversionScale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::Vector3), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                    >, 1usize>("ReadMatrix4x4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadMatrix4x4",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (conversionScale))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadQuaterion(
        &mut self,
        gltfToUnitySpaceRotation: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Quaternion>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::Vector4), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Quaternion>,
                    >, 1usize>("ReadQuaterion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadQuaterion",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Quaternion>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (gltfToUnitySpaceRotation))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                    >, 0usize>("ReadVector2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadVector2",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector3(
        &mut self,
        conversionScale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::Vector3), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                    >, 1usize>("ReadVector3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadVector3",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (conversionScale))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector4(
        &mut self,
        conversionScale: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::Vector4), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                    >, 1usize>("ReadVector4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadVector4",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (conversionScale))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadWeights(
        &mut self,
        resultsBoneWeights: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("ReadWeights")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadWeights",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (resultsBoneWeights))? };
        Ok(__cordl_ret.into())
    }
    pub fn Seek(
        &mut self,
        accessorIndex: i32,
        onlyBufferView: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, bool), quest_hook::libil2cpp::Void, 2usize>("Seek")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Seek",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (accessorIndex, onlyBufferView))? };
        Ok(__cordl_ret.into())
    }
    pub fn SeekStride(
        &mut self,
        strideIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("SeekStride")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SeekStride",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (strideIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToOVRType(
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRGLTFType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::GlobalNamespace::OVRGLTFType,
                        1usize,
                    >("ToOVRType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToOVRType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFType =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryCreate(
        accessorsRoot: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        bufferViewsRoot: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        buffersRoot: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        binaryChunk: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        dataAccessor: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRGLTFAccessor>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRGLTFAccessor>,
                        >,
                    ), bool, 5usize>("TryCreate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryCreate",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    accessorsRoot,
                    bufferViewsRoot,
                    buffersRoot,
                    binaryChunk,
                    dataAccessor,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        accessorsRoot: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        bufferViewsRoot: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        buffersRoot: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        binaryChunkReader: quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
        binaryChinkStart: i32,
        binaryChunkLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                        quest_hook::libil2cpp::Gc<crate::System::IO::BinaryReader>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    accessorsRoot,
                    bufferViewsRoot,
                    buffersRoot,
                    binaryChunkReader,
                    binaryChinkStart,
                    binaryChunkLength,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRGLTFAccessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRGLTFAccessor")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::OVRGLTFAccessor {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRGLTFAccessor")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::OVRGLTFAccessor {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFAccessor")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRGLTFAccessor_GLTFAccessor {
    pub Type: crate::GlobalNamespace::OVRGLTFType,
    pub ComponentType: crate::GlobalNamespace::OVRGLTFComponentType,
    pub ComponentTypeStride: i32,
    pub BufferViewIndex: i32,
    pub ByteOffset: i32,
    pub Count: i32,
    pub Min: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
    pub Max: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFAccessor")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRGLTFAccessor_GLTFAccessor {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFAccessor/GLTFAccessor";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFAccessor")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFAccessor
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFAccessor")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFAccessor
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFAccessor")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFAccessor
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFAccessor")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRGLTFAccessor_GLTFAccessor {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFAccessor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFAccessor
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRGLTFAccessor+GLTFAccessor")]
impl crate::GlobalNamespace::OVRGLTFAccessor_GLTFAccessor {}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBuffer")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRGLTFAccessor_GLTFBuffer {
    pub ByteLength: i32,
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBuffer")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFAccessor/GLTFBuffer";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBuffer")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBuffer
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBuffer")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBuffer")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBuffer {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBuffer
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRGLTFAccessor+GLTFBuffer")]
impl crate::GlobalNamespace::OVRGLTFAccessor_GLTFBuffer {}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBufferView")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRGLTFAccessor_GLTFBufferView {
    pub BufferIndex: i32,
    pub ByteOffset: i32,
    pub ByteLength: i32,
    pub ByteStride: i32,
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBufferView")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBufferView {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFAccessor/GLTFBufferView";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBufferView")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBufferView
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBufferView")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBufferView
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBufferView")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBufferView
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBufferView")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBufferView
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_OVRGLTFAccessor+GLTFBufferView")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRGLTFAccessor_GLTFBufferView
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRGLTFAccessor+GLTFBufferView")]
impl crate::GlobalNamespace::OVRGLTFAccessor_GLTFBufferView {}
