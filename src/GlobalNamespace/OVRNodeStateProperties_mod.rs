#[cfg(feature = "cordl_class_OVRNodeStateProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRNodeStateProperties {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRNodeStateProperties")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRNodeStateProperties {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRNodeStateProperties";
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
#[cfg(feature = "OVRNodeStateProperties")]
impl std::ops::Deref for crate::GlobalNamespace::OVRNodeStateProperties {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNodeStateProperties")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRNodeStateProperties {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::XRNode,
                        crate::GlobalNamespace::NodeStatePropertyType,
                        crate::GlobalNamespace::OVRPlugin_Node,
                        crate::GlobalNamespace::OVRPlugin_Step,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
                    ), bool, 5usize>("GetNodeStatePropertyQuaternion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetNodeStatePropertyQuaternion",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (nodeType, propertyType, ovrpNodeType, stepType, retQuat),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeStatePropertyVector3(
        nodeType: crate::UnityEngine::XR::XRNode,
        propertyType: crate::GlobalNamespace::NodeStatePropertyType,
        ovrpNodeType: crate::GlobalNamespace::OVRPlugin_Node,
        stepType: crate::GlobalNamespace::OVRPlugin_Step,
        retVec: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::XRNode,
                        crate::GlobalNamespace::NodeStatePropertyType,
                        crate::GlobalNamespace::OVRPlugin_Node,
                        crate::GlobalNamespace::OVRPlugin_Step,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    ), bool, 5usize>("GetNodeStatePropertyVector3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetNodeStatePropertyVector3",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (nodeType, propertyType, ovrpNodeType, stepType, retVec))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnityXRNodeStateQuaternion(
        nodeType: crate::UnityEngine::XR::XRNode,
        propertyType: crate::GlobalNamespace::NodeStatePropertyType,
        retQuat: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::XRNode,
                        crate::GlobalNamespace::NodeStatePropertyType,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
                    ), bool, 3usize>("GetUnityXRNodeStateQuaternion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetUnityXRNodeStateQuaternion",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (nodeType, propertyType, retQuat))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnityXRNodeStateVector3(
        nodeType: crate::UnityEngine::XR::XRNode,
        propertyType: crate::GlobalNamespace::NodeStatePropertyType,
        retVec: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::XRNode,
                        crate::GlobalNamespace::NodeStatePropertyType,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                    ), bool, 3usize>("GetUnityXRNodeStateVector3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetUnityXRNodeStateVector3",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (nodeType, propertyType, retVec))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsHmdPresent() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("IsHmdPresent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsHmdPresent",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateProperty(
        nodeType: crate::UnityEngine::XR::XRNode,
        requestedNodeState: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::XR::XRNodeState>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::XRNode,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::XR::XRNodeState>,
                    ), bool, 2usize>("ValidateProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ValidateProperty",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (nodeType, requestedNodeState))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRNodeStateProperties")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRNodeStateProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
