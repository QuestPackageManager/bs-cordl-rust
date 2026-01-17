#[cfg(feature = "cordl_class_ILevelPackProductDataContainer_2")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct ILevelPackProductDataContainer_2<
    TLevelPackProductData: quest_hook::libil2cpp::Type,
    TLevelProductData: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TLevelPackProductData: std::marker::PhantomData<TLevelPackProductData>,
    __cordl_phantom_TLevelProductData: std::marker::PhantomData<TLevelProductData>,
}
#[cfg(feature = "cordl_class_ILevelPackProductDataContainer_2")]
unsafe impl<
        TLevelPackProductData: quest_hook::libil2cpp::Type,
        TLevelProductData: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        TLevelPackProductData,
        TLevelProductData,
    >
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ILevelPackProductDataContainer`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "ILevelPackProductDataContainer`2")
                .unwrap()
                .make_generic::<(TLevelPackProductData, TLevelProductData)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "ILevelPackProductDataContainer_2")]
impl<
        TLevelPackProductData: quest_hook::libil2cpp::Type,
        TLevelProductData: quest_hook::libil2cpp::Type,
    > std::ops::Deref
    for crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        TLevelPackProductData,
        TLevelProductData,
    >
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ILevelPackProductDataContainer_2")]
impl<
        TLevelPackProductData: quest_hook::libil2cpp::Type,
        TLevelProductData: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut
    for crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        TLevelPackProductData,
        TLevelProductData,
    >
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ILevelPackProductDataContainer_2")]
impl<
        TLevelPackProductData: quest_hook::libil2cpp::Type,
        TLevelProductData: quest_hook::libil2cpp::Type,
    >
    crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        TLevelPackProductData,
        TLevelProductData,
    >
{
    pub fn SetLevelPackProductData(
        &mut self,
        newProductPack: TLevelPackProductData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TLevelPackProductData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TLevelProductData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(TLevelPackProductData), quest_hook::libil2cpp::Void, 1usize>(
                        "SetLevelPackProductData",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetLevelPackProductData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (newProductPack))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_levelPackProductData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<TLevelPackProductData>
    where
        TLevelPackProductData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TLevelProductData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), TLevelPackProductData, 0usize>("get_levelPackProductData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_levelPackProductData",
                            0usize
                        )
                    })
            });
        let __cordl_ret: TLevelPackProductData =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ILevelPackProductDataContainer_2")]
impl<
        TLevelPackProductData: quest_hook::libil2cpp::Type,
        TLevelProductData: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        TLevelPackProductData,
        TLevelProductData,
    >
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
