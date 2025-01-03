#[cfg(feature = "UnityOpus+Decoder")]
#[repr(C)]
#[derive(Debug)]
pub struct Decoder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub decoder: crate::System::IntPtr,
    pub channels: crate::UnityOpus::NumChannels,
    pub softclipMem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub disposedValue: bool,
}
#[cfg(feature = "UnityOpus+Decoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityOpus::Decoder => "UnityOpus"."Decoder"
);
#[cfg(feature = "UnityOpus+Decoder")]
impl std::ops::Deref for crate::UnityOpus::Decoder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityOpus+Decoder")]
impl std::ops::DerefMut for crate::UnityOpus::Decoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityOpus+Decoder")]
impl crate::UnityOpus::Decoder {
    pub const maximumPacketDuration: i32 = 5760i32;
    pub fn Decode(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dataLength: i32,
        pcm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        decodeFec: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Decode", (data, dataLength, pcm, decodeFec))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        samplingFrequency: crate::UnityOpus::SamplingFrequency,
        channels: crate::UnityOpus::NumChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (samplingFrequency, channels))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        samplingFrequency: crate::UnityOpus::SamplingFrequency,
        channels: crate::UnityOpus::NumChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (samplingFrequency, channels))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityOpus+Decoder")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityOpus::Decoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityOpus+Decoder")]
impl AsRef<crate::System::IDisposable> for crate::UnityOpus::Decoder {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityOpus+Decoder")]
impl AsMut<crate::System::IDisposable> for crate::UnityOpus::Decoder {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
