#[cfg(feature = "System+Security+Cryptography+SHA1Managed")]
#[repr(C)]
#[derive(Debug)]
pub struct SHA1Managed {
    __cordl_parent: crate::System::Security::Cryptography::SHA1,
    pub _buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _count: i64,
    pub _stateSHA1: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub _expandedBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "System+Security+Cryptography+SHA1Managed")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::SHA1Managed =>
    "System.Security.Cryptography"."SHA1Managed"
);
#[cfg(feature = "System+Security+Cryptography+SHA1Managed")]
impl std::ops::Deref for crate::System::Security::Cryptography::SHA1Managed {
    type Target = crate::System::Security::Cryptography::SHA1;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+SHA1Managed")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::SHA1Managed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+SHA1Managed")]
impl crate::System::Security::Cryptography::SHA1Managed {
    pub fn InitializeState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeState", ())?;
        Ok(__cordl_ret)
    }
    pub fn _EndHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("_EndHash", ())?;
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
    pub fn _HashData(
        &mut self,
        partIn: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        ibStart: i32,
        cbSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_HashData", (partIn, ibStart, cbSize))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+SHA1Managed")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::SHA1Managed {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
