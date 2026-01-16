#[cfg(feature = "cordl_class_Org+BouncyCastle+Math+EC+ScaleYNegateXPointMap")]
#[repr(C)]
#[derive(Debug)]
pub struct ScaleYNegateXPointMap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub scale: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Math+EC+ScaleYNegateXPointMap")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Org::BouncyCastle::Math::EC::ScaleYNegateXPointMap
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC";
    const CLASS_NAME: &'static str = "ScaleYNegateXPointMap";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleYNegateXPointMap")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::ScaleYNegateXPointMap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleYNegateXPointMap")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::ScaleYNegateXPointMap {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleYNegateXPointMap")]
impl crate::Org::BouncyCastle::Math::EC::ScaleYNegateXPointMap {
    pub fn Map(
        &mut self,
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        1usize,
                    >("Map")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Map",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint> =
            unsafe { cordl_method_info.invoke_unchecked(self, (p))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        scale: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scale))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        scale: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (scale))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Math+EC+ScaleYNegateXPointMap")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Org::BouncyCastle::Math::EC::ScaleYNegateXPointMap
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleYNegateXPointMap")]
impl AsRef<crate::Org::BouncyCastle::Math::EC::ECPointMap>
    for crate::Org::BouncyCastle::Math::EC::ScaleYNegateXPointMap
{
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Math::EC::ECPointMap {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ScaleYNegateXPointMap")]
impl AsMut<crate::Org::BouncyCastle::Math::EC::ECPointMap>
    for crate::Org::BouncyCastle::Math::EC::ScaleYNegateXPointMap
{
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Math::EC::ECPointMap {
        unsafe { std::mem::transmute(self) }
    }
}
