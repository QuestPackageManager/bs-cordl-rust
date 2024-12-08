#[cfg(feature = "UnityEngine+Experimental+Audio+AudioSampleProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSampleProvider {
    __cordl_parent: crate::System::Object,
    pub sampleFramesAvailable: *mut crate::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler,
    pub sampleFramesOverflow: *mut crate::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler,
}
#[cfg(feature = "UnityEngine+Experimental+Audio+AudioSampleProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Audio::AudioSampleProvider =>
    "UnityEngine.Experimental.Audio"."AudioSampleProvider"
);
#[cfg(feature = "UnityEngine+Experimental+Audio+AudioSampleProvider")]
impl std::ops::Deref for crate::UnityEngine::Experimental::Audio::AudioSampleProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Audio+AudioSampleProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::Audio::AudioSampleProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Audio+AudioSampleProvider")]
impl crate::UnityEngine::Experimental::Audio::AudioSampleProvider {
    #[cfg(
        feature = "UnityEngine+Experimental+Audio+AudioSampleProvider+SampleFramesHandler"
    )]
    pub type SampleFramesHandler = crate::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler;
    pub fn InvokeSampleFramesAvailable(
        &mut self,
        sampleFrameCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSampleFramesAvailable", (sampleFrameCount))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSampleFramesOverflow(
        &mut self,
        droppedSampleFrameCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSampleFramesOverflow", (droppedSampleFrameCount))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Experimental+Audio+AudioSampleProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::Audio::AudioSampleProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Audio+AudioSampleProvider+SampleFramesHandler"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSampleProvider_SampleFramesHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "UnityEngine+Experimental+Audio+AudioSampleProvider+SampleFramesHandler"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler =>
    "UnityEngine.Experimental.Audio"."AudioSampleProvider/SampleFramesHandler"
);
#[cfg(
    feature = "UnityEngine+Experimental+Audio+AudioSampleProvider+SampleFramesHandler"
)]
impl std::ops::Deref
for crate::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Audio+AudioSampleProvider+SampleFramesHandler"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Audio+AudioSampleProvider+SampleFramesHandler"
)]
impl crate::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler {
    pub fn Invoke(
        &mut self,
        provider: *mut crate::UnityEngine::Experimental::Audio::AudioSampleProvider,
        sampleFrameCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (provider, sampleFrameCount))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Audio+AudioSampleProvider+SampleFramesHandler"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
