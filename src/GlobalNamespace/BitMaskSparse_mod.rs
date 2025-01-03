#[cfg(feature = "BitMaskSparse")]
#[repr(C)]
#[derive(Debug)]
pub struct BitMaskSparse {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bitCount_k__BackingField: i32,
    pub _sparseSet: *mut crate::System::Collections::Generic::SortedSet_1<u32>,
}
#[cfg(feature = "BitMaskSparse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BitMaskSparse => ""
    ."BitMaskSparse"
);
#[cfg(feature = "BitMaskSparse")]
impl std::ops::Deref for crate::GlobalNamespace::BitMaskSparse {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BitMaskSparse")]
impl std::ops::DerefMut for crate::GlobalNamespace::BitMaskSparse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BitMaskSparse")]
impl crate::GlobalNamespace::BitMaskSparse {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BitMaskSparse>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBits(
        &mut self,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("GetBits", (offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bitCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bitCount))?;
        Ok(__cordl_object.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBits(
        &mut self,
        offset: i32,
        bits: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BitMaskSparse>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BitMaskSparse,
        > = __cordl_object.invoke("SetBits", (offset, bits))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bitCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bitCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bitCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_bitCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BitMaskSparse")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BitMaskSparse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BitMaskSparse")]
impl AsRef<
    crate::GlobalNamespace::IBitMask_1<*mut crate::GlobalNamespace::BitMaskSparse>,
> for crate::GlobalNamespace::BitMaskSparse {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IBitMask_1<
        *mut crate::GlobalNamespace::BitMaskSparse,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BitMaskSparse")]
impl AsMut<
    crate::GlobalNamespace::IBitMask_1<*mut crate::GlobalNamespace::BitMaskSparse>,
> for crate::GlobalNamespace::BitMaskSparse {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IBitMask_1<
        *mut crate::GlobalNamespace::BitMaskSparse,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BitMaskSparse")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::BitMaskSparse {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BitMaskSparse")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::BitMaskSparse {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BitMaskSparse")]
impl AsRef<crate::System::IEquatable_1<*mut crate::GlobalNamespace::BitMaskSparse>>
for crate::GlobalNamespace::BitMaskSparse {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<*mut crate::GlobalNamespace::BitMaskSparse> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BitMaskSparse")]
impl AsMut<crate::System::IEquatable_1<*mut crate::GlobalNamespace::BitMaskSparse>>
for crate::GlobalNamespace::BitMaskSparse {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<*mut crate::GlobalNamespace::BitMaskSparse> {
        unsafe { std::mem::transmute(self) }
    }
}
