#[cfg(feature = "Interpolation")]
#[repr(C)]
#[derive(Debug)]
pub struct Interpolation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Interpolation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Interpolation => ""
    ."Interpolation"
);
#[cfg(feature = "Interpolation")]
impl std::ops::Deref for crate::GlobalNamespace::Interpolation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Interpolation")]
impl std::ops::DerefMut for crate::GlobalNamespace::Interpolation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Interpolation")]
impl crate::GlobalNamespace::Interpolation {
    pub fn Interpolate(
        t: f32,
        easeType: crate::GlobalNamespace::EaseType,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Interpolate", (t, easeType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Interpolation")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Interpolation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
