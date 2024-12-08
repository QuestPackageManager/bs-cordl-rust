#[cfg(feature = "Newtonsoft+Json+Utilities+Base64Encoder")]
#[repr(C)]
#[derive(Debug)]
pub struct Base64Encoder {
    __cordl_parent: crate::System::Object,
    pub _charsLine: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _writer: *mut crate::System::IO::TextWriter,
    pub _leftOverBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _leftOverBytesCount: i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+Base64Encoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::Base64Encoder =>
    "Newtonsoft.Json.Utilities"."Base64Encoder"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+Base64Encoder")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::Base64Encoder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+Base64Encoder")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::Base64Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+Base64Encoder")]
impl crate::Newtonsoft::Json::Utilities::Base64Encoder {
    pub const Base64LineSize: i32 = 76i32;
    pub const LineSizeInBytes: i32 = 57i32;
    #[cfg(feature = "Newtonsoft+Json+Utilities+Base64Encoder+_EncodeAsync_d__13")]
    pub type _EncodeAsync_d__13 = crate::Newtonsoft::Json::Utilities::Base64Encoder__EncodeAsync_d__13;
    pub fn Encode(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn EncodeAsync(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("EncodeAsync", (buffer, index, count, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret)
    }
    pub fn FlushAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("FlushAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn FulfillFromLeftover(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FulfillFromLeftover", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        writer: *mut crate::System::IO::TextWriter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (writer))?;
        Ok(__cordl_object)
    }
    pub fn StoreLeftOverBytes(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StoreLeftOverBytes", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateEncode(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateEncode", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn WriteChars(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteChars", (chars, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn WriteCharsAsync(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("WriteCharsAsync", (chars, index, count, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        writer: *mut crate::System::IO::TextWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (writer))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+Base64Encoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::Base64Encoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
