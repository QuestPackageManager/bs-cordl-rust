#[cfg(feature = "SongPackMask")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SongPackMask {
    pub _bloomFilter: crate::GlobalNamespace::BitMask256,
}
#[cfg(feature = "SongPackMask")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SongPackMask {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SongPackMask";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::SongPackMask {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::SongPackMask {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::SongPackMask {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::SongPackMask {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "SongPackMask")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::SongPackMask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SongPackMask")]
impl crate::GlobalNamespace::SongPackMask {
    pub const kHashBits: i32 = 15i32;
    pub const kHashCount: i32 = 2i32;
    pub const kToStringPrefix: &'static str = "[SongPackMask ";
    pub const kToStringSuffix: &'static str = "]";
    pub fn Contains(
        &mut self,
        other: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromSerializedData(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn DifferenceFrom(
        &mut self,
        other: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DifferenceFrom",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_SongPackMask0(
        &mut self,
        other: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBytes(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBytes", (bytes, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        stringSerializedMask: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (stringSerializedMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToShortString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToShortString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse(
        stringSerializedMask: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        songPackMask: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::SongPackMask,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (stringSerializedMask, songPackMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BitMask256_1(
        &mut self,
        bloomFilter: crate::GlobalNamespace::BitMask256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (bloomFilter),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEnumerable_1_2(
        &mut self,
        packs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (packs),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (packId),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_all() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::SongPackMask,
    > {
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_all", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd(
        a: crate::GlobalNamespace::SongPackMask,
        b: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr(
        a: crate::GlobalNamespace::SongPackMask,
        b: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: crate::GlobalNamespace::SongPackMask,
        b: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: crate::GlobalNamespace::SongPackMask,
        b: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SongPackMask")]
impl AsRef<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::SongPackMask,
    >,
> for crate::GlobalNamespace::SongPackMask {
    fn as_ref(
        &self,
    ) -> &crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::SongPackMask,
    > {
        todo!()
    }
}
#[cfg(feature = "SongPackMask")]
impl AsMut<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::SongPackMask,
    >,
> for crate::GlobalNamespace::SongPackMask {
    fn as_mut(
        &mut self,
    ) -> &mut crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::SongPackMask,
    > {
        todo!()
    }
}
#[cfg(feature = "SongPackMask")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::SongPackMask>>
for crate::GlobalNamespace::SongPackMask {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::SongPackMask> {
        todo!()
    }
}
#[cfg(feature = "SongPackMask")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::SongPackMask>>
for crate::GlobalNamespace::SongPackMask {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::SongPackMask> {
        todo!()
    }
}
