#[cfg(feature = "cordl_class_Mono+Math+Prime+PrimalityTests")]
#[repr(C)]
#[derive(Debug)]
pub struct PrimalityTests {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Mono+Math+Prime+PrimalityTests")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Math::Prime::PrimalityTests {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Math.Prime";
    const CLASS_NAME: &'static str = "PrimalityTests";
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
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
impl std::ops::Deref for crate::Mono::Math::Prime::PrimalityTests {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
impl std::ops::DerefMut for crate::Mono::Math::Prime::PrimalityTests {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
impl crate::Mono::Math::Prime::PrimalityTests {
    pub fn GetSPPRounds(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        confidence: crate::Mono::Math::Prime::ConfidenceFactor,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                            crate::Mono::Math::Prime::ConfidenceFactor,
                        ),
                        i32,
                        2usize,
                    >("GetSPPRounds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSPPRounds", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (bi, confidence))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RabinMillerTest(
        n: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        confidence: crate::Mono::Math::Prime::ConfidenceFactor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                            crate::Mono::Math::Prime::ConfidenceFactor,
                        ),
                        bool,
                        2usize,
                    >("RabinMillerTest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RabinMillerTest", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (n, confidence))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Mono+Math+Prime+PrimalityTests")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Math::Prime::PrimalityTests {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
