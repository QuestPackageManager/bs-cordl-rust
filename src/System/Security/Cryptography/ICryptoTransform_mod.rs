#[cfg(feature = "System+Security+Cryptography+ICryptoTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct ICryptoTransform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Security+Cryptography+ICryptoTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::ICryptoTransform
    => "System.Security.Cryptography"."ICryptoTransform"
);
#[cfg(feature = "System+Security+Cryptography+ICryptoTransform")]
impl std::ops::Deref for crate::System::Security::Cryptography::ICryptoTransform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+ICryptoTransform")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::ICryptoTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+ICryptoTransform")]
impl crate::System::Security::Cryptography::ICryptoTransform {
    pub fn TransformBlock(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
        outputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outputOffset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "TransformBlock",
                (inputBuffer, inputOffset, inputCount, outputBuffer, outputOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TransformFinalBlock(
        &mut self,
        inputBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOffset: i32,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("TransformFinalBlock", (inputBuffer, inputOffset, inputCount))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_CanTransformMultipleBlocks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CanTransformMultipleBlocks", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_InputBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OutputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_OutputBlockSize", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+Cryptography+ICryptoTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::ICryptoTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
