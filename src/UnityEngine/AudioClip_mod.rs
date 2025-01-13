#[cfg(feature = "UnityEngine+AudioClip")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioClip {
    __cordl_parent: crate::UnityEngine::Object,
    pub m_PCMReaderCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AudioClip_PCMReaderCallback,
    >,
    pub m_PCMSetPositionCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AudioClip_PCMSetPositionCallback,
    >,
}
#[cfg(feature = "UnityEngine+AudioClip")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::AudioClip {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AudioClip";
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
#[cfg(feature = "UnityEngine+AudioClip")]
impl std::ops::Deref for crate::UnityEngine::AudioClip {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioClip")]
impl std::ops::DerefMut for crate::UnityEngine::AudioClip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioClip")]
impl crate::UnityEngine::AudioClip {
    #[cfg(feature = "UnityEngine+AudioClip+PCMReaderCallback")]
    pub type PCMReaderCallback = crate::UnityEngine::AudioClip_PCMReaderCallback;
    #[cfg(feature = "UnityEngine+AudioClip+PCMSetPositionCallback")]
    pub type PCMSetPositionCallback = crate::UnityEngine::AudioClip_PCMSetPositionCallback;
    pub fn Construct_Internal() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Construct_Internal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUserSound(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lengthSamples: i32,
        channels: i32,
        frequency: i32,
        stream: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CreateUserSound",
                (name, lengthSamples, channels, frequency, stream),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_AudioClip_PCMReaderCallback4(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lengthSamples: i32,
        channels: i32,
        frequency: i32,
        stream: bool,
        pcmreadercallback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AudioClip_PCMReaderCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (name, lengthSamples, channels, frequency, stream, pcmreadercallback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_AudioClip_PCMReaderCallback_AudioClip_PCMSetPositionCallback5(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lengthSamples: i32,
        channels: i32,
        frequency: i32,
        stream: bool,
        pcmreadercallback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AudioClip_PCMReaderCallback,
        >,
        pcmsetpositioncallback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AudioClip_PCMSetPositionCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (
                    name,
                    lengthSamples,
                    channels,
                    frequency,
                    stream,
                    pcmreadercallback,
                    pcmsetpositioncallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Il2CppString_i32_i32_i32__cordl_bool3(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lengthSamples: i32,
        channels: i32,
        frequency: i32,
        stream: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (name, lengthSamples, channels, frequency, stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create__cordl_bool0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lengthSamples: i32,
        channels: i32,
        frequency: i32,
        _3D: bool,
        stream: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (name, lengthSamples, channels, frequency, _3D, stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create__cordl_bool_AudioClip_PCMReaderCallback1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lengthSamples: i32,
        channels: i32,
        frequency: i32,
        _3D: bool,
        stream: bool,
        pcmreadercallback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AudioClip_PCMReaderCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (
                    name,
                    lengthSamples,
                    channels,
                    frequency,
                    _3D,
                    stream,
                    pcmreadercallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create__cordl_bool_AudioClip_PCMReaderCallback_AudioClip_PCMSetPositionCallback2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lengthSamples: i32,
        channels: i32,
        frequency: i32,
        _3D: bool,
        stream: bool,
        pcmreadercallback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AudioClip_PCMReaderCallback,
        >,
        pcmsetpositioncallback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AudioClip_PCMSetPositionCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (
                    name,
                    lengthSamples,
                    channels,
                    frequency,
                    _3D,
                    stream,
                    pcmreadercallback,
                    pcmsetpositioncallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetData_AudioClip_ByRefMut_i32_i32_0(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
        numSamples: i32,
        samplesOffset: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetData", (clip, data, numSamples, samplesOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetData_Il2CppArray_i32_1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        offsetSamples: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetData", (data, offsetSamples))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokePCMReaderCallback_Internal(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokePCMReaderCallback_Internal", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokePCMSetPositionCallback_Internal(
        &mut self,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokePCMSetPositionCallback_Internal", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAudioData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LoadAudioData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetData_AudioClip_Il2CppArray_i32_i32_0(
        clip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        numsamples: i32,
        samplesOffset: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetData", (clip, data, numsamples, samplesOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData_Il2CppArray_i32_1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        offsetSamples: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SetData", (data, offsetSamples))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadAudioData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UnloadAudioData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_m_PCMReaderCallback(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip_PCMReaderCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_m_PCMReaderCallback", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_m_PCMSetPositionCallback(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AudioClip_PCMSetPositionCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_m_PCMSetPositionCallback", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambisonic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ambisonic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_channels(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_channels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_frequency(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_frequency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isReadyToPlay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isReadyToPlay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_length(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_length", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loadInBackground(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_loadInBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loadState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AudioDataLoadState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::AudioDataLoadState = __cordl_object
            .invoke("get_loadState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loadType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AudioClipLoadType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::AudioClipLoadType = __cordl_object
            .invoke("get_loadType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_preloadAudioData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_preloadAudioData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_samples(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_samples", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_m_PCMReaderCallback(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip_PCMReaderCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_m_PCMReaderCallback", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_m_PCMSetPositionCallback(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AudioClip_PCMSetPositionCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_m_PCMSetPositionCallback", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AudioClip")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AudioClip {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AudioClip+PCMReaderCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioClip_PCMReaderCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+AudioClip+PCMReaderCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AudioClip_PCMReaderCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AudioClip/PCMReaderCallback";
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
#[cfg(feature = "UnityEngine+AudioClip+PCMReaderCallback")]
impl std::ops::Deref for crate::UnityEngine::AudioClip_PCMReaderCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioClip+PCMReaderCallback")]
impl std::ops::DerefMut for crate::UnityEngine::AudioClip_PCMReaderCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioClip+PCMReaderCallback")]
impl crate::UnityEngine::AudioClip_PCMReaderCallback {
    pub fn Invoke(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (data))?;
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
#[cfg(feature = "UnityEngine+AudioClip+PCMReaderCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AudioClip_PCMReaderCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AudioClip+PCMSetPositionCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioClip_PCMSetPositionCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+AudioClip+PCMSetPositionCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AudioClip_PCMSetPositionCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AudioClip/PCMSetPositionCallback";
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
#[cfg(feature = "UnityEngine+AudioClip+PCMSetPositionCallback")]
impl std::ops::Deref for crate::UnityEngine::AudioClip_PCMSetPositionCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioClip+PCMSetPositionCallback")]
impl std::ops::DerefMut for crate::UnityEngine::AudioClip_PCMSetPositionCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioClip+PCMSetPositionCallback")]
impl crate::UnityEngine::AudioClip_PCMSetPositionCallback {
    pub fn Invoke(
        &mut self,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (position))?;
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
#[cfg(feature = "UnityEngine+AudioClip+PCMSetPositionCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AudioClip_PCMSetPositionCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
