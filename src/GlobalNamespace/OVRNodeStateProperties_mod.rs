#[cfg(feature = "OVRNodeStateProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRNodeStateProperties {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "OVRNodeStateProperties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRNodeStateProperties => ""
    ."OVRNodeStateProperties"
);
#[cfg(feature = "OVRNodeStateProperties")]
impl std::ops::Deref for crate::GlobalNamespace::OVRNodeStateProperties {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNodeStateProperties")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRNodeStateProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNodeStateProperties")]
impl crate::GlobalNamespace::OVRNodeStateProperties {
    pub fn GetNodeStatePropertyQuaternion(
        nodeType: crate::UnityEngine::XR::XRNode,
        propertyType: crate::GlobalNamespace::NodeStatePropertyType,
        ovrpNodeType: crate::GlobalNamespace::OVRPlugin_Node,
        stepType: crate::GlobalNamespace::OVRPlugin_Step,
        retQuat: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetNodeStatePropertyQuaternion",
                (nodeType, propertyType, ovrpNodeType, stepType, retQuat),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeStatePropertyVector3(
        nodeType: crate::UnityEngine::XR::XRNode,
        propertyType: crate::GlobalNamespace::NodeStatePropertyType,
        ovrpNodeType: crate::GlobalNamespace::OVRPlugin_Node,
        stepType: crate::GlobalNamespace::OVRPlugin_Step,
        retVec: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetNodeStatePropertyVector3",
                (nodeType, propertyType, ovrpNodeType, stepType, retVec),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnityXRNodeStateQuaternion(
        nodeType: crate::UnityEngine::XR::XRNode,
        propertyType: crate::GlobalNamespace::NodeStatePropertyType,
        retQuat: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnityXRNodeStateQuaternion", (nodeType, propertyType, retQuat))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnityXRNodeStateVector3(
        nodeType: crate::UnityEngine::XR::XRNode,
        propertyType: crate::GlobalNamespace::NodeStatePropertyType,
        retVec: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnityXRNodeStateVector3", (nodeType, propertyType, retVec))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHmdPresent() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHmdPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateProperty(
        nodeType: crate::UnityEngine::XR::XRNode,
        requestedNodeState: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::XR::XRNodeState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateProperty", (nodeType, requestedNodeState))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRNodeStateProperties")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRNodeStateProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
