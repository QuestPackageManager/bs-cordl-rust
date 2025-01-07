#[cfg(feature = "QuaternionSerializable")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuaternionSerializable {
    pub _a: i32,
    pub _b: i32,
    pub _c: i32,
}
#[cfg(feature = "QuaternionSerializable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::QuaternionSerializable {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "QuaternionSerializable";
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
#[cfg(feature = "QuaternionSerializable")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::QuaternionSerializable {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "QuaternionSerializable")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::QuaternionSerializable {
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
#[cfg(feature = "QuaternionSerializable")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::QuaternionSerializable {
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
#[cfg(feature = "QuaternionSerializable")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::QuaternionSerializable {
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
#[cfg(feature = "QuaternionSerializable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::QuaternionSerializable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "QuaternionSerializable")]
impl crate::GlobalNamespace::QuaternionSerializable {
    pub const kInvScale: f32 = 0.00008632201f32;
    pub const kOneOverSqrtTwo: f32 = 0.70710677f32;
    pub const kScale: f32 = 11584.53f32;
    pub const kSqrtTwo: f32 = 1.4142135f32;
    pub fn Approximately(
        &mut self,
        other: crate::GlobalNamespace::QuaternionSerializable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Approximately",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Deserialize",
            (reader),
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
    pub fn Equals_QuaternionSerializable0(
        &mut self,
        other: crate::GlobalNamespace::QuaternionSerializable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSmallest(
        sa: i32,
        sb: i32,
        sc: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromSmallest", (sa, sb, sc))?;
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
    pub fn GetSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSize",
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
    pub fn ToSmallest(
        q: crate::UnityEngine::Quaternion,
        sa: quest_hook::libil2cpp::ByRefMut<i32>,
        sb: quest_hook::libil2cpp::ByRefMut<i32>,
        sc: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSmallest", (q, sa, sb, sc))?;
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
    pub fn _ctor_NetDataReader1(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Quaternion0(
        &mut self,
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (q),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_identity() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::QuaternionSerializable,
    > {
        let __cordl_ret: crate::GlobalNamespace::QuaternionSerializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_identity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        a: crate::GlobalNamespace::QuaternionSerializable,
        b: crate::GlobalNamespace::QuaternionSerializable,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::QuaternionSerializable> {
        let __cordl_ret: crate::GlobalNamespace::QuaternionSerializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Quaternion1(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::QuaternionSerializable> {
        let __cordl_ret: crate::GlobalNamespace::QuaternionSerializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_QuaternionSerializable0(
        q: crate::GlobalNamespace::QuaternionSerializable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        a: crate::GlobalNamespace::QuaternionSerializable,
        b: crate::GlobalNamespace::QuaternionSerializable,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::QuaternionSerializable> {
        let __cordl_ret: crate::GlobalNamespace::QuaternionSerializable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "QuaternionSerializable")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::QuaternionSerializable {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "QuaternionSerializable")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::QuaternionSerializable {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        todo!()
    }
}
#[cfg(feature = "QuaternionSerializable")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::QuaternionSerializable>>
for crate::GlobalNamespace::QuaternionSerializable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::QuaternionSerializable> {
        todo!()
    }
}
#[cfg(feature = "QuaternionSerializable")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::QuaternionSerializable>>
for crate::GlobalNamespace::QuaternionSerializable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::GlobalNamespace::QuaternionSerializable,
    > {
        todo!()
    }
}
