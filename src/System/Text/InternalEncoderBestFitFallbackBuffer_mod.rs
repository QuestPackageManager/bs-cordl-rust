#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalEncoderBestFitFallbackBuffer {
    __cordl_parent: crate::System::Text::EncoderFallbackBuffer,
    pub _cBestFit: char,
    pub _oFallback: quest_hook::libil2cpp::Gc<
        crate::System::Text::InternalEncoderBestFitFallback,
    >,
    pub _iCount: i32,
    pub _iSize: i32,
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallbackBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Text::InternalEncoderBestFitFallbackBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text";
    const CLASS_NAME: &'static str = "InternalEncoderBestFitFallbackBuffer";
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetNextChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("GetNextChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MovePrevious(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MovePrevious", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        fallback: quest_hook::libil2cpp::Gc<
            crate::System::Text::InternalEncoderBestFitFallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fallback))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryBestFit(&mut self, cUnknown: char) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("TryBestFit", (cUnknown))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        fallback: quest_hook::libil2cpp::Gc<
            crate::System::Text::InternalEncoderBestFitFallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalSyncObject() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InternalSyncObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Remaining(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Remaining", ())?;
        Ok(__cordl_ret.into())
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
