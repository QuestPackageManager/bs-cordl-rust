#[cfg(feature = "UnityEngine+BoundsInt")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoundsInt {
    pub m_Position: crate::UnityEngine::Vector3Int,
    pub m_Size: crate::UnityEngine::Vector3Int,
}
#[cfg(feature = "UnityEngine+BoundsInt")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::BoundsInt {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "BoundsInt";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::BoundsInt {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::BoundsInt {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::BoundsInt {
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
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::BoundsInt {
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
#[cfg(feature = "UnityEngine+BoundsInt")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::BoundsInt {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+BoundsInt")]
impl crate::UnityEngine::BoundsInt {
    #[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
    pub type PositionEnumerator = crate::UnityEngine::BoundsInt_PositionEnumerator;
    pub fn Equals_BoundsInt1(
        &mut self,
        other: crate::UnityEngine::BoundsInt,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        _cordl_size: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (position, _cordl_size),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allPositionsWithin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::BoundsInt_PositionEnumerator,
    > {
        let __cordl_ret: crate::UnityEngine::BoundsInt_PositionEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_allPositionsWithin",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_max(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        let __cordl_ret: crate::UnityEngine::Vector3Int = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_max",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_min(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        let __cordl_ret: crate::UnityEngine::Vector3Int = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_min",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        let __cordl_ret: crate::UnityEngine::Vector3Int = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_position",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_size(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        let __cordl_ret: crate::UnityEngine::Vector3Int = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_size",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xMax(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xMax",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xMin(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xMin",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yMax(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yMax",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yMin(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yMin",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zMax(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zMax",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zMin(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zMin",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_max(
        &mut self,
        value: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_max",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_min(
        &mut self,
        value: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_min",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_position(
        &mut self,
        value: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_position",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_size(
        &mut self,
        value: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_size",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xMax(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xMax",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xMin(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xMin",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yMax(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yMax",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yMin(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yMin",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zMax(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zMax",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zMin(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zMin",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+BoundsInt")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::BoundsInt>>
for crate::UnityEngine::BoundsInt {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::BoundsInt> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+BoundsInt")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::BoundsInt>>
for crate::UnityEngine::BoundsInt {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::BoundsInt> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+BoundsInt")]
impl AsRef<crate::System::IFormattable> for crate::UnityEngine::BoundsInt {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+BoundsInt")]
impl AsMut<crate::System::IFormattable> for crate::UnityEngine::BoundsInt {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoundsInt_PositionEnumerator {
    pub _min: crate::UnityEngine::Vector3Int,
    pub _max: crate::UnityEngine::Vector3Int,
    pub _current: crate::UnityEngine::Vector3Int,
}
#[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::BoundsInt_PositionEnumerator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "PositionEnumerator";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::BoundsInt_PositionEnumerator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::BoundsInt_PositionEnumerator {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::BoundsInt_PositionEnumerator {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::BoundsInt_PositionEnumerator {
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
#[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::BoundsInt_PositionEnumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
impl crate::UnityEngine::BoundsInt_PositionEnumerator {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::BoundsInt_PositionEnumerator,
    > {
        let __cordl_ret: crate::UnityEngine::BoundsInt_PositionEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IDisposable.Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        min: crate::UnityEngine::Vector3Int,
        max: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        let __cordl_ret: crate::UnityEngine::Vector3Int = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerator_1<crate::UnityEngine::Vector3Int>,
> for crate::UnityEngine::BoundsInt_PositionEnumerator {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::Vector3Int,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerator_1<crate::UnityEngine::Vector3Int>,
> for crate::UnityEngine::BoundsInt_PositionEnumerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::Vector3Int,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::BoundsInt_PositionEnumerator {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::BoundsInt_PositionEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::BoundsInt_PositionEnumerator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+BoundsInt+PositionEnumerator")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::BoundsInt_PositionEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
