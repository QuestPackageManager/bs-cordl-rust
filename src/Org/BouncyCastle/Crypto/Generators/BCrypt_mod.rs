#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+BCrypt")]
#[repr(C)]
#[derive(Debug)]
pub struct BCrypt {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub S: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub P: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+BCrypt")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Generators::BCrypt =>
    "Org.BouncyCastle.Crypto.Generators"."BCrypt"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+BCrypt")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Generators::BCrypt {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+BCrypt")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Generators::BCrypt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+BCrypt")]
impl crate::Org::BouncyCastle::Crypto::Generators::BCrypt {
    pub const MAGIC_STRING_LENGTH: i32 = 6i32;
    pub const MAX_COST: i32 = 31i32;
    pub const MAX_PASSWORD_BYTES: i32 = 72i32;
    pub const MIN_COST: i32 = 4i32;
    pub const P_SZ: i32 = 18i32;
    pub const ROUNDS: i32 = 16i32;
    pub const SALT_SIZE_BYTES: i32 = 16i32;
    pub const SBOX_SK: i32 = 256i32;
    pub const SBOX_SK2: i32 = 512i32;
    pub const SBOX_SK3: i32 = 768i32;
    pub fn CyclicXorKey(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CyclicXorKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn DeriveRawKey(
        &mut self,
        cost: i32,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        psw: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("DeriveRawKey", (cost, salt, psw))?;
        Ok(__cordl_ret)
    }
    pub fn EncryptMagicString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("EncryptMagicString", ())?;
        Ok(__cordl_ret)
    }
    pub fn F(&mut self, x: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("F", (x))?;
        Ok(__cordl_ret)
    }
    pub fn InitState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitState", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessTable(
        &mut self,
        xl: u32,
        xr: u32,
        table: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessTable", (xl, xr, table))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessTableWithSalt(
        &mut self,
        table: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        salt32Bit: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        iv1: u32,
        iv2: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessTableWithSalt", (table, salt32Bit, iv1, iv2))?;
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
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+BCrypt")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::BCrypt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
