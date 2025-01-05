#[cfg(feature = "System+Drawing+KnownColorTable")]
#[repr(C)]
#[derive(Debug)]
pub struct KnownColorTable {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Drawing+KnownColorTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Drawing::KnownColorTable =>
    "System.Drawing"."KnownColorTable"
);
#[cfg(feature = "System+Drawing+KnownColorTable")]
impl std::ops::Deref for crate::System::Drawing::KnownColorTable {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Drawing+KnownColorTable")]
impl std::ops::DerefMut for crate::System::Drawing::KnownColorTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Drawing+KnownColorTable")]
impl crate::System::Drawing::KnownColorTable {
    pub fn EnsureColorNameTable() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureColorNameTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureColorTable() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureColorTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitColorNameTable() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitColorNameTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitColorTable() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitColorTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn KnownColorToArgb(
        color: crate::System::Drawing::KnownColor,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("KnownColorToArgb", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn KnownColorToName(
        color: crate::System::Drawing::KnownColor,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("KnownColorToName", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSystemColors(
        colorTable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateSystemColors", (colorTable))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Drawing+KnownColorTable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Drawing::KnownColorTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
