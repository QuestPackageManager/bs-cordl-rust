#[cfg(feature = "UnityEngine+Timeline+DiscreteTime")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DiscreteTime {
    pub m_DiscreteTime: i64,
}
#[cfg(feature = "UnityEngine+Timeline+DiscreteTime")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Timeline::DiscreteTime {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "DiscreteTime";
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
for crate::UnityEngine::Timeline::DiscreteTime {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Timeline::DiscreteTime {
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
for crate::UnityEngine::Timeline::DiscreteTime {
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
for crate::UnityEngine::Timeline::DiscreteTime {
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
#[cfg(feature = "UnityEngine+Timeline+DiscreteTime")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Timeline::DiscreteTime {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Timeline+DiscreteTime")]
impl crate::UnityEngine::Timeline::DiscreteTime {
    pub const k_Tick: f64 = 0.000000000001f64;
    pub fn CompareTo(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DoubleToDiscreteTime(_cordl_time: f64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoubleToDiscreteTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_DiscreteTime0(
        &mut self,
        other: crate::UnityEngine::Timeline::DiscreteTime,
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
    pub fn FloatToDiscreteTime(_cordl_time: f32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FloatToDiscreteTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromTicks(
        ticks: i64,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromTicks", (ticks))?;
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
    pub fn GetNearestTick(_cordl_time: f64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNearestTick", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTick(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTick",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IntToDiscreteTime(_cordl_time: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntToDiscreteTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max(
        lhs: crate::UnityEngine::Timeline::DiscreteTime,
        rhs: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min(
        lhs: crate::UnityEngine::Timeline::DiscreteTime,
        rhs: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn OneTickAfter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OneTickAfter",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn OneTickBefore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OneTickBefore",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SnapToNearestTick_f32_1(
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SnapToNearestTick", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn SnapToNearestTick_f64_0(
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SnapToNearestTick", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble(_cordl_time: i64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFloat(_cordl_time: i64) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToFloat", (_cordl_time))?;
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
    pub fn _ctor_DiscreteTime0(
        &mut self,
        _cordl_time: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_3(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_2(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_4(
        &mut self,
        _cordl_time: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_f64_5(
        &mut self,
        frame: i32,
        fps: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (frame, fps),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_1(
        &mut self,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tickValue() -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_tickValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        lhs: crate::UnityEngine::Timeline::DiscreteTime,
        rhs: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::Timeline::DiscreteTime,
        rhs: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_DiscreteTime0(
        b: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_DiscreteTime1(
        b: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_DiscreteTime2(
        b: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f32_4(
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_3(
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_i64_5(
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        lhs: crate::UnityEngine::Timeline::DiscreteTime,
        rhs: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual(
        lhs: crate::UnityEngine::Timeline::DiscreteTime,
        rhs: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        _cordl_time: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::Timeline::DiscreteTime,
        rhs: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        lhs: crate::UnityEngine::Timeline::DiscreteTime,
        rhs: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual(
        lhs: crate::UnityEngine::Timeline::DiscreteTime,
        rhs: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        lhs: crate::UnityEngine::Timeline::DiscreteTime,
        rhs: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+DiscreteTime")]
impl AsRef<crate::System::IComparable> for crate::UnityEngine::Timeline::DiscreteTime {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Timeline+DiscreteTime")]
impl AsMut<crate::System::IComparable> for crate::UnityEngine::Timeline::DiscreteTime {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
