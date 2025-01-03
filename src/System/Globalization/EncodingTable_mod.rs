#[cfg(feature = "System+Globalization+EncodingTable")]
#[repr(C)]
#[derive(Debug)]
pub struct EncodingTable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+EncodingTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::EncodingTable =>
    "System.Globalization"."EncodingTable"
);
#[cfg(feature = "System+Globalization+EncodingTable")]
impl std::ops::Deref for crate::System::Globalization::EncodingTable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+EncodingTable")]
impl std::ops::DerefMut for crate::System::Globalization::EncodingTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+EncodingTable")]
impl crate::System::Globalization::EncodingTable {
    pub fn ENC(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cp: u16,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::InternalEncodingDataItem,
    > {
        let __cordl_ret: crate::System::Globalization::InternalEncodingDataItem = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ENC", (name, cp))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCodePageDataItem(
        codepage: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CodePageDataItem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CodePageDataItem,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCodePageDataItem", (codepage))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCodePageFromName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCodePageFromName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNumEncodingItems() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNumEncodingItems", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MapCodePageDataItem(
        cp: u16,
        fcp: u16,
        names: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::InternalCodePageDataItem,
    > {
        let __cordl_ret: crate::System::Globalization::InternalCodePageDataItem = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MapCodePageDataItem", (cp, fcp, names, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetCodePageFromName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("internalGetCodePageFromName", (name))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+EncodingTable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::EncodingTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
