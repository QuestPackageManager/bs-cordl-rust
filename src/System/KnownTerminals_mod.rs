#[cfg(feature = "System+KnownTerminals")]
#[repr(C)]
#[derive(Debug)]
pub struct KnownTerminals {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+KnownTerminals")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::KnownTerminals => "System"
    ."KnownTerminals"
);
#[cfg(feature = "System+KnownTerminals")]
impl std::ops::Deref for crate::System::KnownTerminals {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+KnownTerminals")]
impl std::ops::DerefMut for crate::System::KnownTerminals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+KnownTerminals")]
impl crate::System::KnownTerminals {
    pub fn get_ansi() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_ansi", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_linux() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_linux", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xterm() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_xterm", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+KnownTerminals")]
impl quest_hook::libil2cpp::ObjectType for crate::System::KnownTerminals {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
