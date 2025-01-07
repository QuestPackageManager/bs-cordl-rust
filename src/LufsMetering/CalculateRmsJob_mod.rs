#[cfg(feature = "LufsMetering+CalculateRmsJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalculateRmsJob {
    pub inputData: crate::Unity::Collections::NativeArray_1<f32>,
    pub outputData: crate::Unity::Collections::NativeArray_1<f32>,
    pub step: f32,
    pub timeGate: f32,
    pub rate: i32,
}
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
unsafe impl quest_hook::libil2cpp::Type for crate::LufsMetering::CalculateRmsJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "LufsMetering";
    const CLASS_NAME: &'static str = "CalculateRmsJob";
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
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
unsafe impl quest_hook::libil2cpp::Argument for crate::LufsMetering::CalculateRmsJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::LufsMetering::CalculateRmsJob {
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
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
unsafe impl quest_hook::libil2cpp::Returned for crate::LufsMetering::CalculateRmsJob {
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
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
unsafe impl quest_hook::libil2cpp::Return for crate::LufsMetering::CalculateRmsJob {
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
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LufsMetering::CalculateRmsJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
impl crate::LufsMetering::CalculateRmsJob {
    pub fn Execute(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        inputData: crate::Unity::Collections::NativeArray_1<f32>,
        outputData: crate::Unity::Collections::NativeArray_1<f32>,
        step: f32,
        timeGate: f32,
        rate: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (inputData, outputData, step, timeGate, rate),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
for crate::LufsMetering::CalculateRmsJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
for crate::LufsMetering::CalculateRmsJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
