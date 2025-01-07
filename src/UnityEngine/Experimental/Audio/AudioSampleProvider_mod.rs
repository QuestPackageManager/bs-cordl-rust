#[cfg(feature = "UnityEngine+Experimental+Audio+AudioSampleProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSampleProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sampleFramesAvailable: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler,
    >,
    pub sampleFramesOverflow: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler,
    >,
}
#[cfg(feature = "UnityEngine+Experimental+Audio+AudioSampleProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::Audio::AudioSampleProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Audio";
    const CLASS_NAME: &'static str = "AudioSampleProvider";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Experimental+Audio+AudioSampleProvider")]
impl std::ops::Deref for crate::UnityEngine::Experimental::Audio::AudioSampleProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::Audio::AudioSampleProvider_SampleFramesHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Audio";
    const CLASS_NAME: &'static str = "SampleFramesHandler";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        provider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Audio::AudioSampleProvider,
        >,
        sampleFrameCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (provider, sampleFrameCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
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
