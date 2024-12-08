#[cfg(feature = "Tweening+FrameParityExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FrameParityExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Tweening+FrameParityExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tweening::FrameParityExtensions => "Tweening"
    ."FrameParityExtensions"
);
#[cfg(feature = "Tweening+FrameParityExtensions")]
impl std::ops::Deref for crate::Tweening::FrameParityExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+FrameParityExtensions")]
impl std::ops::DerefMut for crate::Tweening::FrameParityExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tweening+FrameParityExtensions")]
impl crate::Tweening::FrameParityExtensions {}
#[cfg(feature = "Tweening+FrameParityExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Tweening::FrameParityExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
