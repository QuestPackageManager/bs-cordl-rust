#[cfg(feature = "System+Security+Cryptography+DESTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct DESTransform {
    __cordl_parent: crate::Mono::Security::Cryptography::SymmetricTransform,
    pub keySchedule: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub byteBuff: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub dwordBuff: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "System+Security+Cryptography+DESTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::DESTransform =>
    "System.Security.Cryptography"."DESTransform"
);
#[cfg(feature = "System+Security+Cryptography+DESTransform")]
impl std::ops::Deref for crate::System::Security::Cryptography::DESTransform {
    type Target = crate::Mono::Security::Cryptography::SymmetricTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+DESTransform")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::DESTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+DESTransform")]
impl crate::System::Security::Cryptography::DESTransform {
    pub fn BSwap(
        byteBuff: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BSwap", (byteBuff))?;
        Ok(__cordl_ret.into())
    }
    pub fn CipherFunct(&mut self, r: u32, n: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("CipherFunct", (r, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ECB(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ECB", (input, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStrongKey() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetStrongKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        symmAlgo: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::SymmetricAlgorithm,
        >,
        encryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (symmAlgo, encryption, key, iv))?;
        Ok(__cordl_object.into())
    }
    pub fn Permutation(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        permTab: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        preSwap: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Permutation", (input, output, permTab, preSwap))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessBlock(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessBlock", (input, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKey(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        symmAlgo: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::SymmetricAlgorithm,
        >,
        encryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (symmAlgo, encryption, key, iv))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+DESTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::DESTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
