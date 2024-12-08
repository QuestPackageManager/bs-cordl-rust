#[cfg(feature = "AudioReferenceCountingCache")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioReferenceCountingCache {
    __cordl_parent: ReferenceCountingCache_2<
        i32,
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    >,
}
#[cfg(feature = "AudioReferenceCountingCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AudioReferenceCountingCache => ""
    ."AudioReferenceCountingCache"
);
#[cfg(feature = "AudioReferenceCountingCache")]
impl std::ops::Deref for AudioReferenceCountingCache {
    type Target = ReferenceCountingCache_2<
        i32,
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::UnityEngine::AudioClip>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioReferenceCountingCache")]
impl std::ops::DerefMut for AudioReferenceCountingCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioReferenceCountingCache")]
impl AudioReferenceCountingCache {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
}
#[cfg(feature = "AudioReferenceCountingCache")]
impl quest_hook::libil2cpp::ObjectType for AudioReferenceCountingCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
