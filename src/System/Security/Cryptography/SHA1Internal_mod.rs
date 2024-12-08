#[cfg(feature = "System+Security+Cryptography+SHA1Internal")]
#[repr(C)]
#[derive(Debug)]
pub struct SHA1Internal {
    __cordl_parent: crate::System::Object,
    pub _H: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub count: u64,
    pub _ProcessingBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _ProcessingBufferCount: i32,
    pub buff: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "System+Security+Cryptography+SHA1Internal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::SHA1Internal =>
    "System.Security.Cryptography"."SHA1Internal"
);
#[cfg(feature = "System+Security+Cryptography+SHA1Internal")]
impl std::ops::Deref for crate::System::Security::Cryptography::SHA1Internal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+SHA1Internal")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::SHA1Internal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+SHA1Internal")]
impl crate::System::Security::Cryptography::SHA1Internal {
    pub fn HashFinal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("HashFinal", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessBlock(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessBlock", (inputBuffer, inputOffset))?;
        Ok(__cordl_ret)
    }
    pub fn AddLength(
        &mut self,
        length: u64,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLength", (length, buffer, position))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn HashCore(
        &mut self,
        rgb: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        ibStart: i32,
        cbSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HashCore", (rgb, ibStart, cbSize))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessFinalBlock(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessFinalBlock", (inputBuffer, inputOffset, inputCount))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+SHA1Internal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::SHA1Internal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
