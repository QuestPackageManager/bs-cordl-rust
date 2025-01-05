#[cfg(feature = "OVRSceneObjectTransformType")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSceneObjectTransformType {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub TransformType: crate::GlobalNamespace::OVRSceneObjectTransformType_Transformation,
}
#[cfg(feature = "OVRSceneObjectTransformType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSceneObjectTransformType =>
    ""."OVRSceneObjectTransformType"
);
#[cfg(feature = "OVRSceneObjectTransformType")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSceneObjectTransformType {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneObjectTransformType")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSceneObjectTransformType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSceneObjectTransformType")]
impl crate::GlobalNamespace::OVRSceneObjectTransformType {
    #[cfg(feature = "OVRSceneObjectTransformType+Transformation")]
    pub type Transformation = crate::GlobalNamespace::OVRSceneObjectTransformType_Transformation;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSceneObjectTransformType")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSceneObjectTransformType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSceneObjectTransformType+Transformation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRSceneObjectTransformType_Transformation {
    #[default]
    None = 2i32,
    Plane = 1i32,
    Volume = 0i32,
}
#[cfg(feature = "OVRSceneObjectTransformType+Transformation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSceneObjectTransformType_Transformation => ""
    ."OVRSceneObjectTransformType/Transformation"
);
