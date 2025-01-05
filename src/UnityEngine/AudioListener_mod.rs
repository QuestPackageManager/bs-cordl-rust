#[cfg(feature = "UnityEngine+AudioListener")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioListener {
    __cordl_parent: crate::UnityEngine::AudioBehaviour,
}
#[cfg(feature = "UnityEngine+AudioListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioListener => "UnityEngine"
    ."AudioListener"
);
#[cfg(feature = "UnityEngine+AudioListener")]
impl std::ops::Deref for crate::UnityEngine::AudioListener {
    type Target = crate::UnityEngine::AudioBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioListener")]
impl std::ops::DerefMut for crate::UnityEngine::AudioListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioListener")]
impl crate::UnityEngine::AudioListener {
    pub fn GetOutputDataHelper(
        samples: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
        channel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOutputDataHelper", (samples, channel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputData_Il2CppArray1(
        samples: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        channel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOutputData", (samples, channel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputData_i32_0(
        numSamples: i32,
        channel: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOutputData", (numSamples, channel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpectrumDataHelper(
        samples: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
        channel: i32,
        window: crate::UnityEngine::FFTWindow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpectrumDataHelper", (samples, channel, window))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpectrumData_Il2CppArray1(
        samples: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        channel: i32,
        window: crate::UnityEngine::FFTWindow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpectrumData", (samples, channel, window))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpectrumData_i32_0(
        numSamples: i32,
        channel: i32,
        window: crate::UnityEngine::FFTWindow,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpectrumData", (numSamples, channel, window))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_pause() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pause", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_velocityUpdateMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::AudioVelocityUpdateMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::AudioVelocityUpdateMode = __cordl_object
            .invoke("get_velocityUpdateMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_volume() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_volume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pause(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_pause", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_velocityUpdateMode(
        &mut self,
        value: crate::UnityEngine::AudioVelocityUpdateMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_velocityUpdateMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_volume(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_volume", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AudioListener")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AudioListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
