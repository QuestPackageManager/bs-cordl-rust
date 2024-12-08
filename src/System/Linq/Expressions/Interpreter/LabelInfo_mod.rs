#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct LabelInfo {
    __cordl_parent: crate::System::Object,
    pub _node: *mut crate::System::Linq::Expressions::LabelTarget,
    pub _label: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    pub _definitions: *mut crate::System::Object,
    pub _references: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
    >,
    pub _acrossBlockJump: bool,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LabelInfo =>
    "System.Linq.Expressions.Interpreter"."LabelInfo"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelInfo")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::LabelInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelInfo")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Interpreter::LabelInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelInfo")]
impl crate::System::Linq::Expressions::Interpreter::LabelInfo {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+LabelInfo+__c")]
    pub type __c = crate::System::Linq::Expressions::Interpreter::LabelInfo___c;
    pub fn AddDefinition(
        &mut self,
        scope: *mut crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDefinition", (scope))?;
        Ok(__cordl_ret)
    }
    pub fn Define(
        &mut self,
        block: *mut crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Define", (block))?;
        Ok(__cordl_ret)
    }
    pub fn DefinedIn(
        &mut self,
        scope: *mut crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DefinedIn", (scope))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureLabel(
        &mut self,
        compiler: *mut crate::System::Linq::Expressions::Interpreter::LightCompiler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureLabel", (compiler))?;
        Ok(__cordl_ret)
    }
    pub fn FirstDefinition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::LabelScopeInfo = __cordl_object
            .invoke("FirstDefinition", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLabel(
        &mut self,
        compiler: *mut crate::System::Linq::Expressions::Interpreter::LightCompiler,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Interpreter::BranchLabel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Interpreter::BranchLabel = __cordl_object
            .invoke("GetLabel", (compiler))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        node: *mut crate::System::Linq::Expressions::LabelTarget,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node))?;
        Ok(__cordl_object)
    }
    pub fn Reference(
        &mut self,
        block: *mut crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reference", (block))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateJump(
        &mut self,
        reference: *mut crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateJump", (reference))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        node: *mut crate::System::Linq::Expressions::LabelTarget,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasDefinitions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDefinitions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasMultipleDefinitions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasMultipleDefinitions", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LabelInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
