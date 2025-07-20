#[cfg(feature = "OVRGLTFAnimatinonNode")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGLTFAnimatinonNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_intputNodeType: crate::GlobalNamespace::OVRGLTFInputNode,
    pub m_jsonData: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
    pub m_binaryChunk: crate::GlobalNamespace::OVRBinaryChunk,
    pub m_gameObj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub m_inputNodeState: crate::GlobalNamespace::OVRGLTFAnimatinonNode_InputNodeState,
    pub m_morphTargetHandler: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler,
    >,
    pub m_translations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
    >,
    pub m_rotations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Quaternion>,
    >,
    pub m_scales: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
    >,
    pub m_weights: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<f32>,
    >,
    pub m_additiveWeightIndex: i32,
}
#[cfg(feature = "OVRGLTFAnimatinonNode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRGLTFAnimatinonNode {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFAnimatinonNode";
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
#[cfg(feature = "OVRGLTFAnimatinonNode")]
impl std::ops::Deref for crate::GlobalNamespace::OVRGLTFAnimatinonNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRGLTFAnimatinonNode {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode")]
impl crate::GlobalNamespace::OVRGLTFAnimatinonNode {
    #[cfg(feature = "OVRGLTFAnimatinonNode+InputNodeState")]
    pub type InputNodeState = crate::GlobalNamespace::OVRGLTFAnimatinonNode_InputNodeState;
    #[cfg(feature = "OVRGLTFAnimatinonNode+OVRGLTFTransformType")]
    pub type OVRGLTFTransformType = crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType;
    #[cfg(feature = "OVRGLTFAnimatinonNode+OVRInterpolationType")]
    pub type OVRInterpolationType = crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType;
    #[cfg(feature = "OVRGLTFAnimatinonNode+ThumbstickDirection")]
    pub type ThumbstickDirection = crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection;
    pub fn AddChannel(
        &mut self,
        channel: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        samplers: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                            quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddChannel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddChannel", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (channel, samplers))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CloneQuaternion(
        &mut self,
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::Quaternion),
                        crate::UnityEngine::Quaternion,
                        1usize,
                    >("CloneQuaternion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CloneQuaternion", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            method.invoke_unchecked(self, (q))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CloneVector3(
        &mut self,
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::Vector3),
                        crate::UnityEngine::Vector3,
                        1usize,
                    >("CloneVector3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CloneVector3", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, (v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyData<T>(
        &mut self,
        dest: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        >,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::List_1<T>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<T>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CopyData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CopyData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dest, src))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCardinalThumbsticks(
        &mut self,
        joystick: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Tuple_2<
                crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
                crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::Vector2),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Tuple_2<
                                crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
                                crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
                            >,
                        >,
                        1usize,
                    >("GetCardinalThumbsticks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCardinalThumbsticks", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Tuple_2<
                crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
                crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
            >,
        > = unsafe { method.invoke_unchecked(self, (joystick))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCardinalWeights(
        &mut self,
        joystick: crate::UnityEngine::Vector2,
        cardinals: quest_hook::libil2cpp::Gc<
            crate::System::Tuple_2<
                crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
                crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Tuple_2<
                                    crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
                                    crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
                                >,
                            >,
                        ),
                        crate::UnityEngine::Vector2,
                        2usize,
                    >("GetCardinalWeights")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCardinalWeights", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, (joystick, cardinals))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformType(
        &mut self,
        transform: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType,
                        1usize,
                    >("GetTransformType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetTransformType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType = unsafe {
            method.invoke_unchecked(self, (transform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        jsonData: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        binaryChunk: crate::GlobalNamespace::OVRBinaryChunk,
        inputNodeType: crate::GlobalNamespace::OVRGLTFInputNode,
        gameObj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        morphTargetHandler: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (jsonData, binaryChunk, inputNodeType, gameObj, morphTargetHandler),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessAnimationSampler(
        &mut self,
        samplerNode: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        nodeId: i32,
        transformType: crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType,
        extras: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                            i32,
                            crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType,
                            quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ProcessAnimationSampler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ProcessAnimationSampler", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (samplerNode, nodeId, transformType, extras))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToOVRInterpolationType(
        &mut self,
        interpolationType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType,
                        1usize,
                    >("ToOVRInterpolationType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToOVRInterpolationType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType = unsafe {
            method.invoke_unchecked(self, (interpolationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePose_Vector2_2(
        &mut self,
        joystick: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::UnityEngine::Vector2),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdatePose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UpdatePose", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (joystick))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePose__cordl_bool0(
        &mut self,
        down: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdatePose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UpdatePose", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (down))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePose_f32__cordl_bool1(
        &mut self,
        t: f32,
        applyDeadZone: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (f32, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("UpdatePose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UpdatePose", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (t, applyDeadZone))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        jsonData: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
        binaryChunk: crate::GlobalNamespace::OVRBinaryChunk,
        inputNodeType: crate::GlobalNamespace::OVRGLTFInputNode,
        gameObj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        morphTargetHandler: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
                            crate::GlobalNamespace::OVRBinaryChunk,
                            crate::GlobalNamespace::OVRGLTFInputNode,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (jsonData, binaryChunk, inputNodeType, gameObj, morphTargetHandler),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRGLTFAnimatinonNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+InputNodeState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRGLTFAnimatinonNode_InputNodeState {
    pub down: bool,
    pub t: f32,
    pub vecT: crate::UnityEngine::Vector2,
}
#[cfg(feature = "OVRGLTFAnimatinonNode+InputNodeState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_InputNodeState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFAnimatinonNode/InputNodeState";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+InputNodeState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_InputNodeState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+InputNodeState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_InputNodeState {
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
#[cfg(feature = "OVRGLTFAnimatinonNode+InputNodeState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_InputNodeState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+InputNodeState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_InputNodeState {
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
#[cfg(feature = "OVRGLTFAnimatinonNode+InputNodeState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_InputNodeState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+InputNodeState")]
impl crate::GlobalNamespace::OVRGLTFAnimatinonNode_InputNodeState {}
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRGLTFTransformType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRGLTFAnimatinonNode_OVRGLTFTransformType {
    #[default]
    None = 0i32,
    Rotation = 2i32,
    Scale = 3i32,
    Translation = 1i32,
    Weights = 4i32,
}
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRGLTFTransformType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFAnimatinonNode/OVRGLTFTransformType";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRGLTFTransformType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRGLTFTransformType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType {
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
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRGLTFTransformType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRGLTFTransformType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType {
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
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRInterpolationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRGLTFAnimatinonNode_OVRInterpolationType {
    #[default]
    CUBICSPLINE = 3i32,
    LINEAR = 1i32,
    None = 0i32,
    STEP = 2i32,
}
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRInterpolationType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFAnimatinonNode/OVRInterpolationType";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRInterpolationType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRInterpolationType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType {
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
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRInterpolationType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+OVRInterpolationType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType {
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
#[cfg(feature = "OVRGLTFAnimatinonNode+ThumbstickDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRGLTFAnimatinonNode_ThumbstickDirection {
    #[default]
    East = 3i32,
    None = 0i32,
    North = 1i32,
    NorthEast = 2i32,
    NorthWest = 8i32,
    South = 5i32,
    SouthEast = 4i32,
    SouthWest = 6i32,
    West = 7i32,
}
#[cfg(feature = "OVRGLTFAnimatinonNode+ThumbstickDirection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRGLTFAnimatinonNode/ThumbstickDirection";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+ThumbstickDirection")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+ThumbstickDirection")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection {
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
#[cfg(feature = "OVRGLTFAnimatinonNode+ThumbstickDirection")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode+ThumbstickDirection")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection {
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
