#[cfg(feature = "UnityEngine+FixedJoint")]
#[repr(C)]
#[derive(Debug)]
pub struct FixedJoint {
    __cordl_parent: crate::UnityEngine::Joint,
}
#[cfg(feature = "UnityEngine+FixedJoint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FixedJoint => "UnityEngine"
    ."FixedJoint"
);
#[cfg(feature = "UnityEngine+FixedJoint")]
impl std::ops::Deref for crate::UnityEngine::FixedJoint {
    type Target = crate::UnityEngine::Joint;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+FixedJoint")]
impl std::ops::DerefMut for crate::UnityEngine::FixedJoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+FixedJoint")]
impl crate::UnityEngine::FixedJoint {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+FixedJoint")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::FixedJoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
