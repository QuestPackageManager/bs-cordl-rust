#[cfg(feature = "GeometryTools")]
#[repr(C)]
#[derive(Debug)]
pub struct GeometryTools {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GeometryTools")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GeometryTools => ""
    ."GeometryTools"
);
#[cfg(feature = "GeometryTools")]
impl std::ops::Deref for crate::GlobalNamespace::GeometryTools {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GeometryTools")]
impl std::ops::DerefMut for crate::GlobalNamespace::GeometryTools {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GeometryTools")]
impl crate::GlobalNamespace::GeometryTools {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ThreePointsToBox(
        p0: crate::UnityEngine::Vector3,
        p1: crate::UnityEngine::Vector3,
        p2: crate::UnityEngine::Vector3,
        center: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        halfSize: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        orientation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThreePointsToBox", (p0, p1, p2, center, halfSize, orientation))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "GeometryTools")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GeometryTools {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
