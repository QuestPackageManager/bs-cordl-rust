#[cfg(feature = "HoudiniEngineUnity+Test_LayerMask_Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Test_LayerMask_Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+Test_LayerMask_Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::Test_LayerMask_Extensions =>
    "HoudiniEngineUnity"."Test_LayerMask_Extensions"
);
#[cfg(feature = "HoudiniEngineUnity+Test_LayerMask_Extensions")]
impl std::ops::Deref for crate::HoudiniEngineUnity::Test_LayerMask_Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_LayerMask_Extensions")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::Test_LayerMask_Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_LayerMask_Extensions")]
impl crate::HoudiniEngineUnity::Test_LayerMask_Extensions {}
#[cfg(feature = "HoudiniEngineUnity+Test_LayerMask_Extensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::Test_LayerMask_Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
