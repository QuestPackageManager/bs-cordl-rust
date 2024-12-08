#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver+ResolveContext")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StyleVariableResolver_ResolveContext {
    pub sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
    pub handles: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIElements::StyleValueHandle,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver+ResolveContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleVariableResolver_ResolveContext =>
    "UnityEngine.UIElements"."StyleVariableResolver/ResolveContext"
);
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver+ResolveContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleVariableResolver_ResolveContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver+ResolveContext")]
impl crate::UnityEngine::UIElements::StyleVariableResolver_ResolveContext {}
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver+Result")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleVariableResolver_Result {
    Invalid = 1i32,
    NotFound = 2i32,
    Valid = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver+Result")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleVariableResolver_Result => "UnityEngine.UIElements"
    ."StyleVariableResolver/Result"
);
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleVariableResolver {
    __cordl_parent: crate::System::Object,
    pub m_Matcher: *mut crate::UnityEngine::UIElements::StyleSheets::StylePropertyValueMatcher,
    pub m_ResolvedValues: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue,
    >,
    pub m_ResolvedVarStack: *mut crate::System::Collections::Generic::Stack_1<
        *mut crate::System::String,
    >,
    pub m_Property: *mut crate::UnityEngine::UIElements::StyleProperty,
    pub m_ContextStack: *mut crate::System::Collections::Generic::Stack_1<
        crate::UnityEngine::UIElements::StyleVariableResolver_ResolveContext,
    >,
    pub m_CurrentContext: crate::UnityEngine::UIElements::StyleVariableResolver_ResolveContext,
    pub _variableContext_k__BackingField: *mut crate::UnityEngine::UIElements::StyleVariableContext,
}
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleVariableResolver
    => "UnityEngine.UIElements"."StyleVariableResolver"
);
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleVariableResolver {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleVariableResolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver")]
impl crate::UnityEngine::UIElements::StyleVariableResolver {
    pub const kMaxResolves: i32 = 100i32;
    #[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver+ResolveContext")]
    pub type ResolveContext = crate::UnityEngine::UIElements::StyleVariableResolver_ResolveContext;
    #[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver+Result")]
    pub type Result = crate::UnityEngine::UIElements::StyleVariableResolver_Result;
    pub fn AddValue(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        property: *mut crate::UnityEngine::UIElements::StyleProperty,
        sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
        handles: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleValueHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (property, sheet, handles))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PopContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn PushContext(
        &mut self,
        sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
        handles: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleValueHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushContext", (sheet, handles))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveFallback(
        &mut self,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleVariableResolver_Result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleVariableResolver_Result = __cordl_object
            .invoke("ResolveFallback", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveVarFunction_ByRefMut0(
        &mut self,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ResolveVarFunction", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveVarFunction_i32_String1(
        &mut self,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
        argc: i32,
        varName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleVariableResolver_Result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleVariableResolver_Result = __cordl_object
            .invoke("ResolveVarFunction", (index, argc, varName))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveVariable(
        &mut self,
        variableName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleVariableResolver_Result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleVariableResolver_Result = __cordl_object
            .invoke("ResolveVariable", (variableName))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateResolvedValues(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidateResolvedValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentHandles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleValueHandle,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleValueHandle,
        > = __cordl_object.invoke("get_currentHandles", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentSheet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::StyleSheet> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleSheet = __cordl_object
            .invoke("get_currentSheet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_resolvedValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue,
        > = __cordl_object.invoke("get_resolvedValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_variableContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::StyleVariableContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleVariableContext = __cordl_object
            .invoke("get_variableContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_variableContext(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::StyleVariableContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_variableContext", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleVariableResolver")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleVariableResolver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
