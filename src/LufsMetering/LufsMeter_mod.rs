#[cfg(feature = "LufsMetering+LufsMeter")]
#[repr(C)]
#[derive(Debug)]
pub struct LufsMeter {
    __cordl_parent: crate::System::Object,
    pub _inputDataNative: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::Unity::Collections::NativeArray_1<f32>,
    >,
    pub _outputDataNative: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::Unity::Collections::NativeArray_1<f32>,
    >,
}
#[cfg(feature = "LufsMetering+LufsMeter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LufsMetering::LufsMeter => "LufsMetering"
    ."LufsMeter"
);
#[cfg(feature = "LufsMetering+LufsMeter")]
impl std::ops::Deref for crate::LufsMetering::LufsMeter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LufsMetering+LufsMeter")]
impl std::ops::DerefMut for crate::LufsMetering::LufsMeter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LufsMetering+LufsMeter")]
impl crate::LufsMetering::LufsMeter {
    pub const kBlockSize: f32 = 0.4f32;
    #[cfg(feature = "LufsMetering+LufsMeter+__c")]
    pub type __c = crate::LufsMetering::LufsMeter___c;
    pub fn AnalyzeClipLoudness(
        &mut self,
        clip: *mut crate::UnityEngine::AudioClip,
    ) -> quest_hook::libil2cpp::Result<crate::LufsMetering::LoudnessData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LufsMetering::LoudnessData = __cordl_object
            .invoke("AnalyzeClipLoudness", (clip))?;
        Ok(__cordl_ret)
    }
    pub fn IntegratedLoudness(
        &mut self,
        interleavedData: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        numChannels: i32,
        rate: i32,
    ) -> quest_hook::libil2cpp::Result<crate::LufsMetering::LoudnessData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LufsMetering::LoudnessData = __cordl_object
            .invoke("IntegratedLoudness", (interleavedData, numChannels, rate))?;
        Ok(__cordl_ret)
    }
    pub fn MomentaryLoudness(
        &mut self,
        interleavedData: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        numChannels: i32,
        rate: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("MomentaryLoudness", (interleavedData, numChannels, rate))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SwapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwapData", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "LufsMetering+LufsMeter")]
impl quest_hook::libil2cpp::ObjectType for crate::LufsMetering::LufsMeter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}