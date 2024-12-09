#[cfg(feature = "System+Net+WebUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct WebUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+WebUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebUtility => "System.Net"
    ."WebUtility"
);
#[cfg(feature = "System+Net+WebUtility")]
impl std::ops::Deref for crate::System::Net::WebUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebUtility")]
impl std::ops::DerefMut for crate::System::Net::WebUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebUtility")]
impl crate::System::Net::WebUtility {
    #[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
    pub type UrlDecoder = crate::System::Net::WebUtility_UrlDecoder;
}
#[cfg(feature = "System+Net+WebUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
#[repr(C)]
#[derive(Debug)]
pub struct WebUtility_UrlDecoder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bufferSize: i32,
    pub _numChars: i32,
    pub _charBuffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _numBytes: i32,
    pub _byteBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _encoding: *mut crate::System::Text::Encoding,
}
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebUtility_UrlDecoder =>
    "System.Net"."WebUtility/UrlDecoder"
);
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
impl std::ops::Deref for crate::System::Net::WebUtility_UrlDecoder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
impl std::ops::DerefMut for crate::System::Net::WebUtility_UrlDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
impl crate::System::Net::WebUtility_UrlDecoder {
    pub fn AddByte(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddByte", (b))?;
        Ok(__cordl_ret)
    }
    pub fn AddChar(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChar", (ch))?;
        Ok(__cordl_ret)
    }
    pub fn FlushBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetString", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bufferSize: i32,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bufferSize, encoding))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bufferSize: i32,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bufferSize, encoding))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebUtility_UrlDecoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
