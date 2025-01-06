#[cfg(feature = "System+Security+Cryptography+KeySizes")]
#[repr(C)]
#[derive(Debug)]
pub struct KeySizes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_minSize: i32,
    pub m_maxSize: i32,
    pub m_skipSize: i32,
}
#[cfg(feature = "System+Security+Cryptography+KeySizes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::KeySizes =>
    "System.Security.Cryptography"."KeySizes"
);
#[cfg(feature = "System+Security+Cryptography+KeySizes")]
impl std::ops::Deref for crate::System::Security::Cryptography::KeySizes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+KeySizes")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::KeySizes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+KeySizes")]
impl crate::System::Security::Cryptography::KeySizes {
    pub fn IsLegal(&mut self, keySize: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLegal", (keySize))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLegalKeySize(
        legalKeys: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::KeySizes,
                >,
            >,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLegalKeySize", (legalKeys, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        minSize: i32,
        maxSize: i32,
        skipSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (minSize, maxSize, skipSize))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        minSize: i32,
        maxSize: i32,
        skipSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (minSize, maxSize, skipSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MinSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SkipSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SkipSize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+KeySizes")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::KeySizes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
