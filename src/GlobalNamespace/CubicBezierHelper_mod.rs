#[cfg(feature = "CubicBezierHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CubicBezierHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "CubicBezierHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CubicBezierHelper => ""
    ."CubicBezierHelper"
);
#[cfg(feature = "CubicBezierHelper")]
impl std::ops::Deref for crate::GlobalNamespace::CubicBezierHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CubicBezierHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::CubicBezierHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CubicBezierHelper")]
impl crate::GlobalNamespace::CubicBezierHelper {}
#[cfg(feature = "CubicBezierHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CubicBezierHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
