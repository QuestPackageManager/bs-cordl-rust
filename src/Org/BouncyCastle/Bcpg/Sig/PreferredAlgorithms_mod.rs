#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+PreferredAlgorithms")]
#[repr(C)]
#[derive(Debug)]
pub struct PreferredAlgorithms {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+PreferredAlgorithms")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Bcpg::Sig::PreferredAlgorithms => "Org.BouncyCastle.Bcpg.Sig"
    ."PreferredAlgorithms"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+PreferredAlgorithms")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::Sig::PreferredAlgorithms {
    type Target = crate::Org::BouncyCastle::Bcpg::SignatureSubpacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+PreferredAlgorithms")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::Sig::PreferredAlgorithms {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+PreferredAlgorithms")]
impl crate::Org::BouncyCastle::Bcpg::Sig::PreferredAlgorithms {
    pub fn GetPreferences(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("GetPreferences", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray1(
        _cordl_type: crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag,
        critical: bool,
        preferences: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, critical, preferences))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_Il2CppArray0(
        _cordl_type: crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag,
        critical: bool,
        isLongLength: bool,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, critical, isLongLength, data))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        _cordl_type: crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag,
        critical: bool,
        preferences: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, critical, preferences))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_Il2CppArray0(
        &mut self,
        _cordl_type: crate::Org::BouncyCastle::Bcpg::SignatureSubpacketTag,
        critical: bool,
        isLongLength: bool,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, critical, isLongLength, data))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Sig+PreferredAlgorithms")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::Sig::PreferredAlgorithms {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
