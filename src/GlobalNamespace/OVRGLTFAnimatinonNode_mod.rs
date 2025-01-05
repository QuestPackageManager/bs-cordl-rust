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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRGLTFAnimatinonNode => ""
    ."OVRGLTFAnimatinonNode"
);
#[cfg(feature = "OVRGLTFAnimatinonNode")]
impl std::ops::Deref for crate::GlobalNamespace::OVRGLTFAnimatinonNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAnimatinonNode")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRGLTFAnimatinonNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChannel", (channel, samplers))?;
        Ok(__cordl_ret.into())
    }
    pub fn CloneQuaternion(
        &mut self,
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("CloneQuaternion", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn CloneVector3(
        &mut self,
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("CloneVector3", (v))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyData", (dest, src))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Tuple_2<
                crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
                crate::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection,
            >,
        > = __cordl_object.invoke("GetCardinalThumbsticks", (joystick))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetCardinalWeights", (joystick, cardinals))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformType(
        &mut self,
        transform: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType = __cordl_object
            .invoke("GetTransformType", (transform))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessAnimationSampler",
                (samplerNode, nodeId, transformType, extras),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToOVRInterpolationType(
        &mut self,
        interpolationType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType = __cordl_object
            .invoke("ToOVRInterpolationType", (interpolationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePose_Vector2_2(
        &mut self,
        joystick: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePose", (joystick))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePose__cordl_bool0(
        &mut self,
        down: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePose", (down))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePose_f32__cordl_bool1(
        &mut self,
        t: f32,
        applyDeadZone: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePose", (t, applyDeadZone))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (jsonData, binaryChunk, inputNodeType, gameObj, morphTargetHandler),
            )?;
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
#[derive(Debug, Clone, Default)]
pub struct OVRGLTFAnimatinonNode_InputNodeState {
    pub down: bool,
    pub t: f32,
    pub vecT: crate::UnityEngine::Vector2,
}
#[cfg(feature = "OVRGLTFAnimatinonNode+InputNodeState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRGLTFAnimatinonNode_InputNodeState => ""
    ."OVRGLTFAnimatinonNode/InputNodeState"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRGLTFAnimatinonNode_OVRGLTFTransformType => ""
    ."OVRGLTFAnimatinonNode/OVRGLTFTransformType"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRGLTFAnimatinonNode_OVRInterpolationType => ""
    ."OVRGLTFAnimatinonNode/OVRInterpolationType"
);
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRGLTFAnimatinonNode_ThumbstickDirection => ""
    ."OVRGLTFAnimatinonNode/ThumbstickDirection"
);
