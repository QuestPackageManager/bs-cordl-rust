#[cfg(feature = "UnityOpus+Encoder")]
#[repr(C)]
#[derive(Debug)]
pub struct Encoder {
    __cordl_parent: crate::System::Object,
    pub bitrate: i32,
    pub complexity: i32,
    pub signal: crate::UnityOpus::OpusSignal,
    pub encoder: crate::System::IntPtr,
    pub channels: crate::UnityOpus::NumChannels,
    pub disposedValue: bool,
}
#[cfg(feature = "UnityOpus+Encoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityOpus::Encoder => "UnityOpus"."Encoder"
);
#[cfg(feature = "UnityOpus+Encoder")]
impl std::ops::Deref for crate::UnityOpus::Encoder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityOpus+Encoder")]
impl std::ops::DerefMut for crate::UnityOpus::Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityOpus+Encoder")]
impl crate::UnityOpus::Encoder {
    pub fn Encode(
        &mut self,
        pcm: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        count: i32,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Encode", (pcm, count, output))?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Bitrate(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Bitrate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool0(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Bitrate(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Bitrate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Signal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityOpus::OpusSignal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityOpus::OpusSignal = __cordl_object
            .invoke("get_Signal", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        samplingFrequency: crate::UnityOpus::SamplingFrequency,
        channels: crate::UnityOpus::NumChannels,
        application: crate::UnityOpus::OpusApplication,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (samplingFrequency, channels, application))?;
        Ok(__cordl_ret)
    }
    pub fn set_Complexity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Complexity", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Signal(
        &mut self,
        value: crate::UnityOpus::OpusSignal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Signal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Complexity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Complexity", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        samplingFrequency: crate::UnityOpus::SamplingFrequency,
        channels: crate::UnityOpus::NumChannels,
        application: crate::UnityOpus::OpusApplication,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (samplingFrequency, channels, application))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityOpus+Encoder")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityOpus::Encoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
