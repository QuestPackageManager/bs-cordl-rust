#[cfg(feature = "SplineUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct SplineUtils {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "SplineUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SplineUtils => ""."SplineUtils"
);
#[cfg(feature = "SplineUtils")]
impl std::ops::Deref for crate::GlobalNamespace::SplineUtils {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SplineUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::SplineUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SplineUtils")]
impl crate::GlobalNamespace::SplineUtils {
    pub fn Interpolate(
        t0: crate::UnityEngine::Vector3,
        p0: crate::UnityEngine::Vector3,
        p1: crate::UnityEngine::Vector3,
        t1: crate::UnityEngine::Vector3,
        f: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Interpolate", (t0, p0, p1, t1, f))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SplineUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SplineUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
