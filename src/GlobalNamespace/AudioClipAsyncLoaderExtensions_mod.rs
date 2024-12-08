#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioClipAsyncLoaderExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AudioClipAsyncLoaderExtensions => ""
    ."AudioClipAsyncLoaderExtensions"
);
#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
impl std::ops::Deref for AudioClipAsyncLoaderExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
impl std::ops::DerefMut for AudioClipAsyncLoaderExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
impl AudioClipAsyncLoaderExtensions {}
#[cfg(feature = "AudioClipAsyncLoaderExtensions")]
impl quest_hook::libil2cpp::ObjectType for AudioClipAsyncLoaderExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
