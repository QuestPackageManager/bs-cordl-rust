#[cfg(feature = "BitMask256")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BitMask256 {
    pub _d0: u64,
    pub _d1: u64,
    pub _d2: u64,
    pub _d3: u64,
}
#[cfg(feature = "BitMask256")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BitMask256 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BitMask256";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::BitMask256 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::BitMask256 {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::BitMask256 {
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
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::BitMask256 {
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
#[cfg(feature = "BitMask256")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::BitMask256 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BitMask256")]
impl crate::GlobalNamespace::BitMask256 {
    pub fn CreateFromSerializedData(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BitMask256> {
        let __cordl_ret: crate::GlobalNamespace::BitMask256 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CreateFromSerializedData",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BitMask256> {
        let __cordl_ret: crate::GlobalNamespace::BitMask256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_BitMask256_0(
        &mut self,
        other: crate::GlobalNamespace::BitMask256,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
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
    pub fn GetBits(
        &mut self,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBits",
            (offset, count),
        )?;
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
    pub fn SetBits(
        &mut self,
        offset: i32,
        bits: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BitMask256> {
        let __cordl_ret: crate::GlobalNamespace::BitMask256 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetBits",
            (offset, bits),
        )?;
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
    pub fn _ctor_u64_1(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u64_u64_u64_0(
        &mut self,
        d0: u64,
        d1: u64,
        d2: u64,
        d3: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (d0, d1, d2, d3),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bitCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bitCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxValue() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BitMask256,
    > {
        let __cordl_ret: crate::GlobalNamespace::BitMask256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_maxValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd(
        a: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
        b: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BitMask256> {
        let __cordl_ret: crate::GlobalNamespace::BitMask256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr(
        a: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
        b: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BitMask256> {
        let __cordl_ret: crate::GlobalNamespace::BitMask256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
        b: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr(
        a: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
        b: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BitMask256> {
        let __cordl_ret: crate::GlobalNamespace::BitMask256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        value: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BitMask256> {
        let __cordl_ret: crate::GlobalNamespace::BitMask256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
        b: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LeftShift(
        a: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BitMask256> {
        let __cordl_ret: crate::GlobalNamespace::BitMask256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LeftShift", (a, bits))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_RightShift(
        a: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BitMask256>,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BitMask256> {
        let __cordl_ret: crate::GlobalNamespace::BitMask256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_RightShift", (a, bits))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BitMask256")]
impl AsRef<crate::GlobalNamespace::IBitMask_1<crate::GlobalNamespace::BitMask256>>
for crate::GlobalNamespace::BitMask256 {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IBitMask_1<crate::GlobalNamespace::BitMask256> {
        todo!()
    }
}
#[cfg(feature = "BitMask256")]
impl AsMut<crate::GlobalNamespace::IBitMask_1<crate::GlobalNamespace::BitMask256>>
for crate::GlobalNamespace::BitMask256 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IBitMask_1<crate::GlobalNamespace::BitMask256> {
        todo!()
    }
}
#[cfg(feature = "BitMask256")]
impl AsRef<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::BitMask256,
    >,
> for crate::GlobalNamespace::BitMask256 {
    fn as_ref(
        &self,
    ) -> &crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::BitMask256,
    > {
        todo!()
    }
}
#[cfg(feature = "BitMask256")]
impl AsMut<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::BitMask256,
    >,
> for crate::GlobalNamespace::BitMask256 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        crate::GlobalNamespace::BitMask256,
    > {
        todo!()
    }
}
#[cfg(feature = "BitMask256")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::BitMask256>>
for crate::GlobalNamespace::BitMask256 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::BitMask256> {
        todo!()
    }
}
#[cfg(feature = "BitMask256")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::BitMask256>>
for crate::GlobalNamespace::BitMask256 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::BitMask256> {
        todo!()
    }
}
