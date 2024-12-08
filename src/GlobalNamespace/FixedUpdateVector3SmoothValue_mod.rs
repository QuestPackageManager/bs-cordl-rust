#[cfg(feature = "FixedUpdateVector3SmoothValue")]
#[repr(C)]
#[derive(Debug)]
pub struct FixedUpdateVector3SmoothValue {
    __cordl_parent: crate::GlobalNamespace::FixedUpdateSmoothValue_1<
        crate::UnityEngine::Vector3,
    >,
}
#[cfg(feature = "FixedUpdateVector3SmoothValue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FixedUpdateVector3SmoothValue
    => ""."FixedUpdateVector3SmoothValue"
);
#[cfg(feature = "FixedUpdateVector3SmoothValue")]
impl std::ops::Deref for crate::GlobalNamespace::FixedUpdateVector3SmoothValue {
    type Target = crate::GlobalNamespace::FixedUpdateSmoothValue_1<
        crate::UnityEngine::Vector3,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FixedUpdateVector3SmoothValue")]
impl std::ops::DerefMut for crate::GlobalNamespace::FixedUpdateVector3SmoothValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FixedUpdateVector3SmoothValue")]
impl crate::GlobalNamespace::FixedUpdateVector3SmoothValue {
    pub fn Interpolate(
        &mut self,
        value0: crate::UnityEngine::Vector3,
        value1: crate::UnityEngine::Vector3,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("Interpolate", (value0, value1, t))?;
        Ok(__cordl_ret)
    }
    pub fn New(smooth: f32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (smooth))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        smooth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (smooth))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FixedUpdateVector3SmoothValue")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FixedUpdateVector3SmoothValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
