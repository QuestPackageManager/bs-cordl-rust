#[cfg(feature = "cordl_class_JetBrains+Annotations+CollectionAccessAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionAccessAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _CollectionAccessType_k__BackingField: crate::JetBrains::Annotations::CollectionAccessType,
}
#[cfg(feature = "cordl_class_JetBrains+Annotations+CollectionAccessAttribute")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::JetBrains::Annotations::CollectionAccessAttribute
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "JetBrains.Annotations";
    const CLASS_NAME: &'static str = "CollectionAccessAttribute";
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
#[cfg(feature = "JetBrains+Annotations+CollectionAccessAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::CollectionAccessAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+CollectionAccessAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::CollectionAccessAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+CollectionAccessAttribute")]
impl crate::JetBrains::Annotations::CollectionAccessAttribute {
    pub fn New(
        collectionAccessType: crate::JetBrains::Annotations::CollectionAccessType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collectionAccessType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        collectionAccessType: crate::JetBrains::Annotations::CollectionAccessType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::JetBrains::Annotations::CollectionAccessType),
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
            unsafe { cordl_method_info.invoke_unchecked(self, (collectionAccessType))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_CollectionAccessType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::JetBrains::Annotations::CollectionAccessType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::JetBrains::Annotations::CollectionAccessType, 0usize>(
                        "get_CollectionAccessType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_CollectionAccessType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::JetBrains::Annotations::CollectionAccessType =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_CollectionAccessType(
        &mut self,
        value: crate::JetBrains::Annotations::CollectionAccessType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::JetBrains::Annotations::CollectionAccessType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_CollectionAccessType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_CollectionAccessType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_JetBrains+Annotations+CollectionAccessAttribute")]
impl quest_hook::libil2cpp::ObjectType
    for crate::JetBrains::Annotations::CollectionAccessAttribute
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
