#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalEncoderBestFitFallbackBuffer {
    __cordl_parent: crate::System::Text::EncoderFallbackBuffer,
    pub _cBestFit: char,
    pub _oFallback: *mut crate::System::Text::InternalEncoderBestFitFallback,
    pub _iCount: i32,
    pub _iSize: i32,
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::InternalEncoderBestFitFallbackBuffer => "System.Text"
    ."InternalEncoderBestFitFallbackBuffer"
);
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
impl std::ops::Deref for crate::System::Text::InternalEncoderBestFitFallbackBuffer {
    type Target = crate::System::Text::EncoderFallbackBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
impl std::ops::DerefMut for crate::System::Text::InternalEncoderBestFitFallbackBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
impl crate::System::Text::InternalEncoderBestFitFallbackBuffer {
    pub fn TryBestFit(&mut self, cUnknown: char) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("TryBestFit", (cUnknown))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("GetNextChar", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        fallback: *mut crate::System::Text::InternalEncoderBestFitFallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fallback))?;
        Ok(__cordl_ret)
    }
    pub fn get_Remaining(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Remaining", ())?;
        Ok(__cordl_ret)
    }
    pub fn MovePrevious(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MovePrevious", ())?;
        Ok(__cordl_ret)
    }
    pub fn Fallback_i32_0(
        &mut self,
        charUnknown: char,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Fallback", (charUnknown, index))?;
        Ok(__cordl_ret)
    }
    pub fn Fallback__cordl_char_i32_1(
        &mut self,
        charUnknownHigh: char,
        charUnknownLow: char,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Fallback", (charUnknownHigh, charUnknownLow, index))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        fallback: *mut crate::System::Text::InternalEncoderBestFitFallback,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fallback))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::InternalEncoderBestFitFallbackBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
