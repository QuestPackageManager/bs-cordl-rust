#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
#[repr(C)]
#[derive(Debug)]
pub struct Nat {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
unsafe impl quest_hook::libil2cpp::Type for crate::Org::BouncyCastle::Math::Raw::Nat {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.Raw";
    const CLASS_NAME: &'static str = "Nat";
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
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::Raw::Nat {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::Raw::Nat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
impl crate::Org::BouncyCastle::Math::Raw::Nat {
    pub const M: u64 = 4294967295u64;
    pub fn Add(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                4usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Add", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, y, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn Add33At_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                ),
                u32,
                5usize,
            >("Add33At")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Add33At", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Add33At_i32_u32_Il2CppArray_i32_0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                4usize,
            >("Add33At")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Add33At", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Add33To_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                4usize,
            >("Add33To")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Add33To", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Add33To_i32_u32_Il2CppArray0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                3usize,
            >("Add33To")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Add33To", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddBothTo_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                4usize,
            >("AddBothTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddBothTo", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, y, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddBothTo_i32_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                7usize,
            >("AddBothTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddBothTo", 7usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, y, yOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddDWordAt_i32_1(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                ),
                u32,
                5usize,
            >("AddDWordAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddDWordAt", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddDWordAt_i32_u64_Il2CppArray_i32_0(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                4usize,
            >("AddDWordAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddDWordAt", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddDWordTo_i32_1(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                4usize,
            >("AddDWordTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddDWordTo", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddDWordTo_i32_u64_Il2CppArray0(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                3usize,
            >("AddDWordTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddDWordTo", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddToEachOther(
        len: i32,
        u: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        uOff: i32,
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        vOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                5usize,
            >("AddToEachOther")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddToEachOther", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, u, uOff, v, vOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddTo_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                3usize,
            >("AddTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddTo", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddTo_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                5usize,
            >("AddTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddTo", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddTo_i32_Il2CppArray_i32_u32_2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        cIn: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    u32,
                ),
                u32,
                6usize,
            >("AddTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddTo", 6usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, z, zOff, cIn))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddWordAt_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                ),
                u32,
                5usize,
            >("AddWordAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddWordAt", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddWordAt_i32_u32_Il2CppArray_i32_0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                4usize,
            >("AddWordAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddWordAt", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddWordTo_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                4usize,
            >("AddWordTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddWordTo", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddWordTo_i32_u32_Il2CppArray0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                3usize,
            >("AddWordTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "AddWordTo", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn CAdd(
        len: i32,
        mask: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                5usize,
            >("CAdd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "CAdd", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, mask, x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CMov_i32_i32_Il2CppArray_i32_Il2CppArray_i32_0(
        len: i32,
        mask: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("CMov")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "CMov", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, mask, x, xOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CMov_i32_i32_Il2CppArray_i32_Il2CppArray_i32_1(
        len: i32,
        mask: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("CMov")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "CMov", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, mask, x, xOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CSub_Il2CppArray0(
        len: i32,
        mask: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                5usize,
            >("CSub")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "CSub", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, mask, x, y, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CSub_i32_i32_Il2CppArray_i32_1(
        len: i32,
        mask: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                8usize,
            >("CSub")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "CSub", 8usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, mask, x, xOff, y, yOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                3usize,
            >("Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Compare", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn Compare_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                5usize,
            >("Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Compare", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, y, yOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy64_Il2CppArray1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Copy64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Copy64", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, x, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy64_i32_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                2usize,
            >("Copy64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Copy64", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u64>,
        > = unsafe { method.invoke_unchecked((), (len, x))? };
        Ok(__cordl_ret.into())
    }
    pub fn Copy64_i32_Il2CppArray_i32_2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("Copy64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Copy64", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, x, xOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Copy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Copy", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, x, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Copy_i32_Il2CppArray1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                2usize,
            >("Copy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Copy", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = unsafe { method.invoke_unchecked((), (len, x))? };
        Ok(__cordl_ret.into())
    }
    pub fn Copy_i32_Il2CppArray_i32_2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("Copy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Copy", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, x, xOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                1usize,
            >("Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Create", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = unsafe { method.invoke_unchecked((), (len))? };
        Ok(__cordl_ret.into())
    }
    pub fn Create64(
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                1usize,
            >("Create64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Create64", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u64>,
        > = unsafe { method.invoke_unchecked((), (len))? };
        Ok(__cordl_ret.into())
    }
    pub fn DecAt_i32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                ),
                i32,
                4usize,
            >("DecAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "DecAt", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, z, zOff, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DecAt_i32_Il2CppArray_i32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                3usize,
            >("DecAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "DecAt", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, z, zPos))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dec_Il2CppArray1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                3usize,
            >("Dec")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Dec", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, x, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dec_i32_Il2CppArray0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                2usize,
            >("Dec")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Dec", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn Eq(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                bool,
                3usize,
            >("Eq")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Eq", 3usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (len, x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromBigInteger(
        bits: i32,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                2usize,
            >("FromBigInteger")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "FromBigInteger", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = unsafe { method.invoke_unchecked((), (bits, x))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromBigInteger64(
        bits: i32,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                2usize,
            >("FromBigInteger64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "FromBigInteger64", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u64>,
        > = unsafe { method.invoke_unchecked((), (bits, x))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBit(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bit: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                2usize,
            >("GetBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "GetBit", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (x, bit))? };
        Ok(__cordl_ret.into())
    }
    pub fn Gte(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                bool,
                3usize,
            >("Gte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Gte", 3usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (len, x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn IncAt_i32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                ),
                u32,
                4usize,
            >("IncAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "IncAt", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, z, zOff, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IncAt_i32_Il2CppArray_i32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                3usize,
            >("IncAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "IncAt", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, z, zPos))? };
        Ok(__cordl_ret.into())
    }
    pub fn Inc_Il2CppArray1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                3usize,
            >("Inc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Inc", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn Inc_i32_Il2CppArray0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                2usize,
            >("Inc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Inc", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsOne(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                bool,
                2usize,
            >("IsOne")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "IsOne", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (len, x))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsZero(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                bool,
                2usize,
            >("IsZero")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "IsZero", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (len, x))? };
        Ok(__cordl_ret.into())
    }
    pub fn LessThan_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                3usize,
            >("LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "LessThan", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn LessThan_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                5usize,
            >("LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "LessThan", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, y, yOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Mul31BothAdd(
        len: i32,
        a: u32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        b: u32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                7usize,
            >("Mul31BothAdd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Mul31BothAdd", 7usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, a, x, b, y, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MulAddTo_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                4usize,
            >("MulAddTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "MulAddTo", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, y, zz))? };
        Ok(__cordl_ret.into())
    }
    pub fn MulAddTo_i32_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zzOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                7usize,
            >("MulAddTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "MulAddTo", 7usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, y, yOff, zz, zzOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MulWordAddTo(
        len: i32,
        x: u32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                6usize,
            >("MulWordAddTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "MulWordAddTo", 6usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, y, yOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MulWordDwordAddAt(
        len: i32,
        x: u32,
        y: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                5usize,
            >("MulWordDwordAddAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "MulWordDwordAddAt", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, y, z, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MulWord_Il2CppArray0(
        len: i32,
        x: u32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                4usize,
            >("MulWord")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "MulWord", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, y, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn MulWord_i32_Il2CppArray_i32_1(
        len: i32,
        x: u32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                6usize,
            >("MulWord")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "MulWord", 6usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, y, yOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Mul_Il2CppArray_i32_i32_i32_i32_Il2CppArray_i32_2(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        xLen: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        yLen: i32,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zzOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("Mul")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Mul", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (x, xOff, xLen, y, yOff, yLen, zz, zzOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Mul_i32_Il2CppArray_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Mul")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Mul", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, x, y, zz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Mul_i32_Il2CppArray_i32_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zzOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("Mul")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Mul", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, x, xOff, y, yOff, zz, zzOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ShiftDownBit_i32_u32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    u32,
                ),
                u32,
                4usize,
            >("ShiftDownBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftDownBit", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, z, zOff, c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBit_i32_u32_Il2CppArray_i32_3(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                6usize,
            >("ShiftDownBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftDownBit", 6usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, c, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBit_u32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    u32,
                ),
                u32,
                3usize,
            >("ShiftDownBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftDownBit", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, z, c))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBit_u32_Il2CppArray2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                4usize,
            >("ShiftDownBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftDownBit", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, c, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBits_i32_u32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        bits: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                    u32,
                ),
                u32,
                5usize,
            >("ShiftDownBits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftDownBits", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, z, zOff, bits, c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBits_i32_u32_Il2CppArray_i32_3(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        bits: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                7usize,
            >("ShiftDownBits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftDownBits", 7usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, bits, c, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBits_u32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bits: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    u32,
                ),
                u32,
                4usize,
            >("ShiftDownBits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftDownBits", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, z, bits, c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownBits_u32_Il2CppArray2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bits: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                5usize,
            >("ShiftDownBits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftDownBits", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, bits, c, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftDownWord(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    u32,
                ),
                u32,
                3usize,
            >("ShiftDownWord")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftDownWord", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, z, c))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBit64(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        xOff: i32,
        c: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    i32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    i32,
                ),
                u64,
                6usize,
            >("ShiftUpBit64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBit64", 6usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, c, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBit_i32_u32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    u32,
                ),
                u32,
                4usize,
            >("ShiftUpBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBit", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, z, zOff, c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBit_i32_u32_Il2CppArray_i32_3(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                6usize,
            >("ShiftUpBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBit", 6usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, c, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBit_u32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    u32,
                ),
                u32,
                3usize,
            >("ShiftUpBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBit", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, z, c))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBit_u32_Il2CppArray2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                4usize,
            >("ShiftUpBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBit", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (len, x, c, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits64_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        xOff: i32,
        bits: i32,
        c: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    i32,
                    i32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    i32,
                ),
                u64,
                7usize,
            >("ShiftUpBits64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBits64", 7usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, bits, c, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits64_i32_Il2CppArray_i32_i32_u64_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        zOff: i32,
        bits: i32,
        c: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                    i32,
                    i32,
                    u64,
                ),
                u64,
                5usize,
            >("ShiftUpBits64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBits64", 5usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (len, z, zOff, bits, c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits_i32_u32_1(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        bits: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                    u32,
                ),
                u32,
                5usize,
            >("ShiftUpBits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBits", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, z, zOff, bits, c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits_i32_u32_Il2CppArray_i32_3(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        bits: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                7usize,
            >("ShiftUpBits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBits", 7usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, bits, c, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits_u32_0(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bits: i32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    u32,
                ),
                u32,
                4usize,
            >("ShiftUpBits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBits", 4usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, z, bits, c))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShiftUpBits_u32_Il2CppArray2(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        bits: i32,
        c: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                5usize,
            >("ShiftUpBits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ShiftUpBits", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (len, x, bits, c, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SquareWordAddTo_Il2CppArray0(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xPos: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                3usize,
            >("SquareWordAddTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SquareWordAddTo", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (x, xPos, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn SquareWordAddTo_i32_Il2CppArray_i32_1(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        xPos: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                5usize,
            >("SquareWordAddTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SquareWordAddTo", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (x, xOff, xPos, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SquareWordAdd_Il2CppArray0(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xPos: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                u32,
                3usize,
            >("SquareWordAdd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SquareWordAdd", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (x, xPos, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn SquareWordAdd_i32_Il2CppArray_i32_1(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        xPos: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                u32,
                5usize,
            >("SquareWordAdd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SquareWordAdd", 5usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (x, xOff, xPos, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Square_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Square")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Square", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, x, zz))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Square_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        zz: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zzOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("Square")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Square", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, x, xOff, zz, zzOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sub33At_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                ),
                i32,
                5usize,
            >("Sub33At")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Sub33At", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sub33At_i32_u32_Il2CppArray_i32_0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                4usize,
            >("Sub33At")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Sub33At", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sub33From_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                4usize,
            >("Sub33From")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Sub33From", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sub33From_i32_u32_Il2CppArray0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                3usize,
            >("Sub33From")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Sub33From", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, x, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn SubBothFrom_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                4usize,
            >("SubBothFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubBothFrom", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, x, y, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn SubBothFrom_i32_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                7usize,
            >("SubBothFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubBothFrom", 7usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, y, yOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubDWordAt_i32_1(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                ),
                i32,
                5usize,
            >("SubDWordAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubDWordAt", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubDWordAt_i32_u64_Il2CppArray_i32_0(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                4usize,
            >("SubDWordAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubDWordAt", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubDWordFrom_i32_1(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                4usize,
            >("SubDWordFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubDWordFrom", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubDWordFrom_i32_u64_Il2CppArray0(
        len: i32,
        x: u64,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                3usize,
            >("SubDWordFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubDWordFrom", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, x, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn SubFrom_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                3usize,
            >("SubFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubFrom", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, x, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn SubFrom_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                5usize,
            >("SubFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubFrom", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubWordAt_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                ),
                i32,
                5usize,
            >("SubWordAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubWordAt", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubWordAt_i32_u32_Il2CppArray_i32_0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                4usize,
            >("SubWordAt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubWordAt", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubWordFrom_i32_1(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                4usize,
            >("SubWordFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubWordFrom", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubWordFrom_i32_u32_Il2CppArray0(
        len: i32,
        x: u32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                3usize,
            >("SubWordFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "SubWordFrom", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, x, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn Sub_Il2CppArray0(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                i32,
                4usize,
            >("Sub")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Sub", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (len, x, y, z))? };
        Ok(__cordl_ret.into())
    }
    pub fn Sub_i32_i32_Il2CppArray_i32_1(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOff: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOff: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        zOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                i32,
                7usize,
            >("Sub")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Sub", 7usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, x, xOff, y, yOff, z, zOff))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBigInteger(
        len: i32,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                2usize,
            >("ToBigInteger")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "ToBigInteger", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = unsafe { method.invoke_unchecked((), (len, x))? };
        Ok(__cordl_ret.into())
    }
    pub fn Zero(
        len: i32,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Zero")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), "Zero", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (len, z))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Math::Raw::Nat as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Math::Raw::Nat as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Raw+Nat")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Math::Raw::Nat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
