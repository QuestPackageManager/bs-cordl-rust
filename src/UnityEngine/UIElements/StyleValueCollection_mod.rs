#[cfg(feature = "UnityEngine+UIElements+StyleValueCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleValueCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Values: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSheets::StyleValue,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueCollection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleValueCollection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "StyleValueCollection";
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
#[cfg(feature = "UnityEngine+UIElements+StyleValueCollection")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleValueCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueCollection")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleValueCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueCollection")]
impl crate::UnityEngine::UIElements::StyleValueCollection {
    pub fn GetStyleFloat(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::StyleSheets::StylePropertyId),
                crate::UnityEngine::UIElements::StyleFloat,
                1usize,
            >("GetStyleFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStyleFloat", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat = unsafe {
            method.invoke_unchecked(self, (id))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStyleInt(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleInt> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::StyleSheets::StylePropertyId),
                crate::UnityEngine::UIElements::StyleInt,
                1usize,
            >("GetStyleInt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStyleInt", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleInt = unsafe {
            method.invoke_unchecked(self, (id))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStyleLength(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::StyleSheets::StylePropertyId),
                crate::UnityEngine::UIElements::StyleLength,
                1usize,
            >("GetStyleLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStyleLength", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength = unsafe {
            method.invoke_unchecked(self, (id))
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
    pub fn SetStyleValue(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleSheets::StyleValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::StyleSheets::StyleValue),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetStyleValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetStyleValue", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetStyleValue(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleSheets::StyleValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::StyleSheets::StyleValue,
                    >,
                ),
                bool,
                2usize,
            >("TryGetStyleValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetStyleValue", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (id, value)) };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+UIElements+StyleValueCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleValueCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
