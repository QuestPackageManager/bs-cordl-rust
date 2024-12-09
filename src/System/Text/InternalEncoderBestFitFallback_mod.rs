#[cfg(feature = "System+Text+InternalEncoderBestFitFallback")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalEncoderBestFitFallback {
    __cordl_parent: crate::System::Text::EncoderFallback,
    pub _encoding: *mut crate::System::Text::Encoding,
    pub _arrayBestFit: *mut quest_hook::libil2cpp::Il2CppArray<char>,
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::InternalEncoderBestFitFallback =>
    "System.Text"."InternalEncoderBestFitFallback"
);
#[cfg(feature = "System+Text+InternalEncoderBestFitFallback")]
impl std::ops::Deref for crate::System::Text::InternalEncoderBestFitFallback {
    type Target = crate::System::Text::EncoderFallback;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallback")]
impl std::ops::DerefMut for crate::System::Text::InternalEncoderBestFitFallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallback")]
impl crate::System::Text::InternalEncoderBestFitFallback {
    pub fn CreateFallbackBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::EncoderFallbackBuffer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::EncoderFallbackBuffer = __cordl_object
            .invoke("CreateFallbackBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoding))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxCharCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxCharCount", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+InternalEncoderBestFitFallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::InternalEncoderBestFitFallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
