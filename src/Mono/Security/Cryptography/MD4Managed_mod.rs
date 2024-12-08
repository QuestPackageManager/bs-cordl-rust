#[cfg(feature = "Mono+Security+Cryptography+MD4Managed")]
#[repr(C)]
#[derive(Debug)]
pub struct MD4Managed {
    __cordl_parent: crate::Mono::Security::Cryptography::MD4,
    pub state: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub count: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub x: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub digest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Mono+Security+Cryptography+MD4Managed")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Cryptography::MD4Managed =>
    "Mono.Security.Cryptography"."MD4Managed"
);
#[cfg(feature = "Mono+Security+Cryptography+MD4Managed")]
impl std::ops::Deref for crate::Mono::Security::Cryptography::MD4Managed {
    type Target = crate::Mono::Security::Cryptography::MD4;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+MD4Managed")]
impl std::ops::DerefMut for crate::Mono::Security::Cryptography::MD4Managed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Cryptography+MD4Managed")]
impl crate::Mono::Security::Cryptography::MD4Managed {
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
    pub fn FF(
        &mut self,
        a: quest_hook::libil2cpp::ByRefMut<u32>,
        b: u32,
        c: u32,
        d: u32,
        x: u32,
        s: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FF", (a, b, c, d, x, s))?;
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
    pub fn GG(
        &mut self,
        a: quest_hook::libil2cpp::ByRefMut<u32>,
        b: u32,
        c: u32,
        d: u32,
        x: u32,
        s: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GG", (a, b, c, d, x, s))?;
        Ok(__cordl_ret)
    }
    pub fn F(&mut self, x: u32, y: u32, z: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("F", (x, y, z))?;
        Ok(__cordl_ret)
    }
    pub fn ROL(&mut self, x: u32, n: u8) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("ROL", (x, n))?;
        Ok(__cordl_ret)
    }
    pub fn Encode(
        &mut self,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (output, input))?;
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
    pub fn H(&mut self, x: u32, y: u32, z: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("H", (x, y, z))?;
        Ok(__cordl_ret)
    }
    pub fn Decode(
        &mut self,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Decode", (output, input, index))?;
        Ok(__cordl_ret)
    }
    pub fn MD4Transform(
        &mut self,
        state: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        block: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MD4Transform", (state, block, index))?;
        Ok(__cordl_ret)
    }
    pub fn HH(
        &mut self,
        a: quest_hook::libil2cpp::ByRefMut<u32>,
        b: u32,
        c: u32,
        d: u32,
        x: u32,
        s: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HH", (a, b, c, d, x, s))?;
        Ok(__cordl_ret)
    }
    pub fn G(&mut self, x: u32, y: u32, z: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("G", (x, y, z))?;
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
#[cfg(feature = "Mono+Security+Cryptography+MD4Managed")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Cryptography::MD4Managed {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
