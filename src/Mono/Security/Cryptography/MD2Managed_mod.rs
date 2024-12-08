#[cfg(feature = "Mono+Security+Cryptography+MD2Managed")]
#[repr(C)]
#[derive(Debug)]
pub struct MD2Managed {
    __cordl_parent: crate::Mono::Security::Cryptography::MD2,
    pub state: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub checksum: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub count: i32,
    pub x: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Mono+Security+Cryptography+MD2Managed")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Cryptography::MD2Managed =>
    "Mono.Security.Cryptography"."MD2Managed"
);
#[cfg(feature = "Mono+Security+Cryptography+MD2Managed")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::MD2Managed {
    type Target = crate::Mono::Security::Cryptography::MD2;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+MD2Managed")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::MD2Managed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+MD2Managed")]
impl crate::Mono::Security::Cryptography::MD2Managed {
    pub fn Padding(
        &mut self,
        nLength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Padding", (nLength))?;
        Ok(__cordl_ret)
    }
    pub fn HashCore(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        ibStart: i32,
        cbSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HashCore", (array, ibStart, cbSize))?;
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
    pub fn MD2Transform(
        &mut self,
        state: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        checksum: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        block: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MD2Transform", (state, checksum, block, index))?;
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
#[cfg(feature = "Mono+Security+Cryptography+MD2Managed")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::MD2Managed {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
