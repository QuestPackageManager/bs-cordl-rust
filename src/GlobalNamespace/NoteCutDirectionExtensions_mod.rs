#[cfg(feature = "NoteCutDirectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutDirectionExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "NoteCutDirectionExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NoteCutDirectionExtensions => ""
    ."NoteCutDirectionExtensions"
);
#[cfg(feature = "NoteCutDirectionExtensions")]
impl std::ops::Deref for NoteCutDirectionExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutDirectionExtensions")]
impl std::ops::DerefMut for NoteCutDirectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutDirectionExtensions")]
impl NoteCutDirectionExtensions {}
#[cfg(feature = "NoteCutDirectionExtensions")]
impl quest_hook::libil2cpp::ObjectType for NoteCutDirectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
