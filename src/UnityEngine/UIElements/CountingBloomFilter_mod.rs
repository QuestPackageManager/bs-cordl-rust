#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CountingBloomFilter {
    pub m_Counters: crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer,
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::CountingBloomFilter {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "CountingBloomFilter";
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
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::CountingBloomFilter {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::CountingBloomFilter {
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
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::CountingBloomFilter {
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
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::CountingBloomFilter {
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
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::CountingBloomFilter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
impl crate::UnityEngine::UIElements::CountingBloomFilter {
    #[cfg(
        feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer"
    )]
    pub type _m_Counters_e__FixedBuffer = crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer;
    pub fn AdjustSlot(
        &mut self,
        index: u32,
        increment: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::CountingBloomFilter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AdjustSlot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::CountingBloomFilter as
                    quest_hook::libil2cpp::Type > ::class(), "AdjustSlot", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (index, increment))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContainsHash(&mut self, hash: u32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::CountingBloomFilter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), bool, 1usize>("ContainsHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::CountingBloomFilter as
                    quest_hook::libil2cpp::Type > ::class(), "ContainsHash", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (hash))? };
        Ok(__cordl_ret.into())
    }
    pub fn Hash1(&mut self, hash: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::CountingBloomFilter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), u32, 1usize>("Hash1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::CountingBloomFilter as
                    quest_hook::libil2cpp::Type > ::class(), "Hash1", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (hash))? };
        Ok(__cordl_ret.into())
    }
    pub fn Hash2(&mut self, hash: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::CountingBloomFilter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), u32, 1usize>("Hash2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::CountingBloomFilter as
                    quest_hook::libil2cpp::Type > ::class(), "Hash2", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (hash))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertHash(
        &mut self,
        hash: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::CountingBloomFilter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>("InsertHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::CountingBloomFilter as
                    quest_hook::libil2cpp::Type > ::class(), "InsertHash", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (hash))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSlotEmpty(&mut self, index: u32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::CountingBloomFilter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), bool, 1usize>("IsSlotEmpty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::CountingBloomFilter as
                    quest_hook::libil2cpp::Type > ::class(), "IsSlotEmpty", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveHash(
        &mut self,
        hash: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::CountingBloomFilter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>("RemoveHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::CountingBloomFilter as
                    quest_hook::libil2cpp::Type > ::class(), "RemoveHash", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (hash))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CountingBloomFilter__m_Counters_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "CountingBloomFilter/<m_Counters>e__FixedBuffer";
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
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer {
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
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer {
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
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer {
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
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
impl crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer {}
