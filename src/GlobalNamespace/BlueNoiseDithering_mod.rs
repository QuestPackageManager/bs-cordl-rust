#[cfg(feature = "BlueNoiseDithering")]
#[repr(C)]
#[derive(Debug)]
pub struct BlueNoiseDithering {
    __cordl_parent: PersistentScriptableObject,
    pub _noiseTexture: *mut crate::UnityEngine::Texture2D,
}
#[cfg(feature = "BlueNoiseDithering")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BlueNoiseDithering => ""."BlueNoiseDithering"
);
#[cfg(feature = "BlueNoiseDithering")]
impl std::ops::Deref for BlueNoiseDithering {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BlueNoiseDithering")]
impl std::ops::DerefMut for BlueNoiseDithering {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BlueNoiseDithering")]
impl BlueNoiseDithering {
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
    pub fn SetBlueNoiseShaderParams(
        &mut self,
        cameraPixelWidth: i32,
        cameraPixelHeight: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBlueNoiseShaderParams", (cameraPixelWidth, cameraPixelHeight))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BlueNoiseDithering")]
impl quest_hook::libil2cpp::ObjectType for BlueNoiseDithering {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
