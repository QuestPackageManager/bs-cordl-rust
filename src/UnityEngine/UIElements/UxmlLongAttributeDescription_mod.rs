#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlLongAttributeDescription {
    __cordl_parent: crate::UnityEngine::UIElements::TypedUxmlAttributeDescription_1<i64>,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UxmlLongAttributeDescription {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UxmlLongAttributeDescription";
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
#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlLongAttributeDescription {
    type Target = crate::UnityEngine::UIElements::TypedUxmlAttributeDescription_1<i64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UxmlLongAttributeDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
impl crate::UnityEngine::UIElements::UxmlLongAttributeDescription {
    pub fn ConvertValueToLong(
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultValue: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i64),
                i64,
                2usize,
            >("ConvertValueToLong")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ConvertValueToLong", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (v, defaultValue)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetValueFromBag(
        &mut self,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::IUxmlAttributes,
                    >,
                    crate::UnityEngine::UIElements::CreationContext,
                ),
                i64,
                2usize,
            >("GetValueFromBag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetValueFromBag", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, (bag, cc)) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlLongAttributeDescription")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlLongAttributeDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
