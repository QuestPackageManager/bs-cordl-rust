#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleComplexSelector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ancestorHashes: crate::UnityEngine::UIElements::Hashes,
    pub m_Specificity: i32,
    pub _rule_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleRule,
    >,
    pub m_isSimple: bool,
    pub m_Selectors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::UIElements::StyleSelector,
        >,
    >,
    pub ruleIndex: i32,
    pub nextInTable: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleComplexSelector,
    >,
    pub orderInStyleSheet: i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleComplexSelector =>
    "UnityEngine.UIElements"."StyleComplexSelector"
);
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleComplexSelector {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleComplexSelector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector")]
impl crate::UnityEngine::UIElements::StyleComplexSelector {
    #[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector+PseudoStateData")]
    pub type PseudoStateData = crate::UnityEngine::UIElements::StyleComplexSelector_PseudoStateData;
    pub fn CachePseudoStateMasks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CachePseudoStateMasks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateHashes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateHashes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterDeserialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeSerialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StyleSelectorPartCompare(
        x: crate::UnityEngine::UIElements::StyleSelectorPart,
        y: crate::UnityEngine::UIElements::StyleSelectorPart,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StyleSelectorPartCompare", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isSimple(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSimple", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleRule,
        > = __cordl_object.invoke("get_rule", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::UIElements::StyleSelector,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::UIElements::StyleSelector,
            >,
        > = __cordl_object.invoke("get_selectors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_specificity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_specificity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rule(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rule", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectors(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::UIElements::StyleSelector,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectors", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleComplexSelector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector")]
impl AsRef<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::UIElements::StyleComplexSelector {
    fn as_ref(&self) -> &crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector")]
impl AsMut<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::UIElements::StyleComplexSelector {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector+PseudoStateData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StyleComplexSelector_PseudoStateData {
    pub state: crate::UnityEngine::UIElements::PseudoStates,
    pub negate: bool,
}
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector+PseudoStateData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleComplexSelector_PseudoStateData =>
    "UnityEngine.UIElements"."StyleComplexSelector/PseudoStateData"
);
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector+PseudoStateData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleComplexSelector_PseudoStateData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleComplexSelector+PseudoStateData")]
impl crate::UnityEngine::UIElements::StyleComplexSelector_PseudoStateData {
    pub fn _ctor(
        &mut self,
        state: crate::UnityEngine::UIElements::PseudoStates,
        negate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (state, negate),
        )?;
        Ok(__cordl_ret.into())
    }
}
