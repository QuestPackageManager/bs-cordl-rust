#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
#[repr(C)]
#[derive(Debug)]
pub struct Timeout {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub durationMillis: i64,
    pub startMillis: i64,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::Timeout {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "Timeout";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::Timeout {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::Timeout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
impl crate::Org::BouncyCastle::Crypto::Tls::Timeout {
    pub fn ConstrainWaitMillis(
        waitMillis: i32,
        timeout: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Timeout,
        >,
        currentTimeMillis: i64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::Timeout,
                    >,
                    i64,
                ),
                i32,
                3usize,
            >("ConstrainWaitMillis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ConstrainWaitMillis", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (waitMillis, timeout, currentTimeMillis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ForWaitMillis_i32_0(
        waitMillis: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::Timeout>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::Timeout,
                >,
                1usize,
            >("ForWaitMillis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ForWaitMillis", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Timeout,
        > = unsafe { method.invoke_unchecked((), (waitMillis)) };
        Ok(__cordl_ret.into())
    }
    pub fn ForWaitMillis_i64_1(
        waitMillis: i32,
        currentTimeMillis: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::Timeout>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i64),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Tls::Timeout,
                >,
                2usize,
            >("ForWaitMillis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ForWaitMillis", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Timeout,
        > = unsafe { method.invoke_unchecked((), (waitMillis, currentTimeMillis)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetWaitMillis(
        timeout: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Timeout,
        >,
        currentTimeMillis: i64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::Timeout,
                    >,
                    i64,
                ),
                i32,
                2usize,
            >("GetWaitMillis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetWaitMillis", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (timeout, currentTimeMillis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasExpired(
        timeout: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Timeout,
        >,
        currentTimeMillis: i64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::Timeout,
                    >,
                    i64,
                ),
                bool,
                2usize,
            >("HasExpired")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HasExpired", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (timeout, currentTimeMillis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_i64_0(
        durationMillis: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (durationMillis))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i64_1(
        durationMillis: i64,
        currentTimeMillis: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (durationMillis, currentTimeMillis))?;
        Ok(__cordl_object.into())
    }
    pub fn RemainingMillis(
        &mut self,
        currentTimeMillis: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64), i64, 1usize>("RemainingMillis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemainingMillis", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked(self, (currentTimeMillis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_0(
        &mut self,
        durationMillis: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (durationMillis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_1(
        &mut self,
        durationMillis: i64,
        currentTimeMillis: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64, i64), quest_hook::libil2cpp::Void, 2usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (durationMillis, currentTimeMillis))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+Timeout")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::Timeout {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
