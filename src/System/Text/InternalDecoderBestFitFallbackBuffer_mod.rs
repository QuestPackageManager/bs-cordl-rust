#[cfg(feature = "System+Text+InternalDecoderBestFitFallbackBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalDecoderBestFitFallbackBuffer {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Text::DecoderFallbackBuffer,
    >,
    pub _cBestFit: char,
    pub _iCount: i32,
    pub _iSize: i32,
    pub _oFallback: quest_hook::libil2cpp::Gc<
        crate::System::Text::InternalDecoderBestFitFallback,
    >,
}
#[cfg(feature = "System+Text+InternalDecoderBestFitFallbackBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::InternalDecoderBestFitFallbackBuffer => "System.Text"
    ."InternalDecoderBestFitFallbackBuffer"
);
#[cfg(feature = "System+Text+InternalDecoderBestFitFallbackBuffer")]
impl std::ops::Deref for crate::System::Text::InternalDecoderBestFitFallbackBuffer {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Text::DecoderFallbackBuffer>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+InternalDecoderBestFitFallbackBuffer")]
impl std::ops::DerefMut for crate::System::Text::InternalDecoderBestFitFallbackBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+InternalDecoderBestFitFallbackBuffer")]
impl crate::System::Text::InternalDecoderBestFitFallbackBuffer {
    pub fn Fallback(
        &mut self,
        bytesUnknown: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Fallback", (bytesUnknown, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("GetNextChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalFallback(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("InternalFallback", (bytes, pBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        fallback: quest_hook::libil2cpp::Gc<
            crate::System::Text::InternalDecoderBestFitFallback,
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
    pub fn TryBestFit(
        &mut self,
        bytesCheck: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("TryBestFit", (bytesCheck))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        fallback: quest_hook::libil2cpp::Gc<
            crate::System::Text::InternalDecoderBestFitFallback,
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
#[cfg(feature = "System+Text+InternalDecoderBestFitFallbackBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::InternalDecoderBestFitFallbackBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
