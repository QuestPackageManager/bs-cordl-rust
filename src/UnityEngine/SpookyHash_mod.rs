#[cfg(feature = "UnityEngine+SpookyHash")]
#[repr(C)]
#[derive(Debug)]
pub struct SpookyHash {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SpookyHash")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::SpookyHash {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "SpookyHash";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+SpookyHash")]
impl std::ops::Deref for crate::UnityEngine::SpookyHash {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpookyHash")]
impl std::ops::DerefMut for crate::UnityEngine::SpookyHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpookyHash")]
impl crate::UnityEngine::SpookyHash {
    #[cfg(feature = "UnityEngine+SpookyHash+U")]
    pub type U = crate::UnityEngine::SpookyHash_U;
    pub fn AttemptDetectAllowUnalignedRead() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AttemptDetectAllowUnalignedRead", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn End(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        h0: quest_hook::libil2cpp::ByRefMut<u64>,
        h1: quest_hook::libil2cpp::ByRefMut<u64>,
        h2: quest_hook::libil2cpp::ByRefMut<u64>,
        h3: quest_hook::libil2cpp::ByRefMut<u64>,
        h4: quest_hook::libil2cpp::ByRefMut<u64>,
        h5: quest_hook::libil2cpp::ByRefMut<u64>,
        h6: quest_hook::libil2cpp::ByRefMut<u64>,
        h7: quest_hook::libil2cpp::ByRefMut<u64>,
        h8: quest_hook::libil2cpp::ByRefMut<u64>,
        h9: quest_hook::libil2cpp::ByRefMut<u64>,
        h10: quest_hook::libil2cpp::ByRefMut<u64>,
        h11: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("End", (data, h0, h1, h2, h3, h4, h5, h6, h7, h8, h9, h10, h11))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndPartial(
        h0: quest_hook::libil2cpp::ByRefMut<u64>,
        h1: quest_hook::libil2cpp::ByRefMut<u64>,
        h2: quest_hook::libil2cpp::ByRefMut<u64>,
        h3: quest_hook::libil2cpp::ByRefMut<u64>,
        h4: quest_hook::libil2cpp::ByRefMut<u64>,
        h5: quest_hook::libil2cpp::ByRefMut<u64>,
        h6: quest_hook::libil2cpp::ByRefMut<u64>,
        h7: quest_hook::libil2cpp::ByRefMut<u64>,
        h8: quest_hook::libil2cpp::ByRefMut<u64>,
        h9: quest_hook::libil2cpp::ByRefMut<u64>,
        h10: quest_hook::libil2cpp::ByRefMut<u64>,
        h11: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndPartial", (h0, h1, h2, h3, h4, h5, h6, h7, h8, h9, h10, h11))?;
        Ok(__cordl_ret.into())
    }
    pub fn Hash(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: u64,
        hash1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        hash2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Hash", (message, length, hash1, hash2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mix(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        s0: quest_hook::libil2cpp::ByRefMut<u64>,
        s1: quest_hook::libil2cpp::ByRefMut<u64>,
        s2: quest_hook::libil2cpp::ByRefMut<u64>,
        s3: quest_hook::libil2cpp::ByRefMut<u64>,
        s4: quest_hook::libil2cpp::ByRefMut<u64>,
        s5: quest_hook::libil2cpp::ByRefMut<u64>,
        s6: quest_hook::libil2cpp::ByRefMut<u64>,
        s7: quest_hook::libil2cpp::ByRefMut<u64>,
        s8: quest_hook::libil2cpp::ByRefMut<u64>,
        s9: quest_hook::libil2cpp::ByRefMut<u64>,
        s10: quest_hook::libil2cpp::ByRefMut<u64>,
        s11: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mix", (data, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11))?;
        Ok(__cordl_ret.into())
    }
    pub fn Rot64(
        x: quest_hook::libil2cpp::ByRefMut<u64>,
        k: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Rot64", (x, k))?;
        Ok(__cordl_ret.into())
    }
    pub fn Short(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: u64,
        hash1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        hash2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Short", (message, length, hash1, hash2))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShortEnd(
        h0: quest_hook::libil2cpp::ByRefMut<u64>,
        h1: quest_hook::libil2cpp::ByRefMut<u64>,
        h2: quest_hook::libil2cpp::ByRefMut<u64>,
        h3: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShortEnd", (h0, h1, h2, h3))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShortMix(
        h0: quest_hook::libil2cpp::ByRefMut<u64>,
        h1: quest_hook::libil2cpp::ByRefMut<u64>,
        h2: quest_hook::libil2cpp::ByRefMut<u64>,
        h3: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShortMix", (h0, h1, h2, h3))?;
        Ok(__cordl_ret.into())
    }
    pub fn memset(
        dst: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: i32,
        numberOfBytes: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("memset", (dst, value, numberOfBytes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SpookyHash")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SpookyHash {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+SpookyHash+U")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpookyHash_U {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
}
#[cfg(feature = "UnityEngine+SpookyHash+U")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::SpookyHash_U {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "U";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::SpookyHash_U {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::SpookyHash_U {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::SpookyHash_U {
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
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::SpookyHash_U {
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
#[cfg(feature = "UnityEngine+SpookyHash+U")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::SpookyHash_U {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SpookyHash+U")]
impl crate::UnityEngine::SpookyHash_U {
    pub fn _ctor(
        &mut self,
        p8: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (p8),
        )?;
        Ok(__cordl_ret.into())
    }
}
