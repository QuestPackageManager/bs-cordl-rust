#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct LabelInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    pub _label: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::BranchLabel,
    >,
    pub _definitions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _references: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
            >,
        >,
    >,
    pub _acrossBlockJump: bool,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::Interpreter::LabelInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "LabelInfo";
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
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelInfo")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::LabelInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn AddDefinition(
        &mut self,
        scope: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDefinition", (scope))?;
        Ok(__cordl_ret.into())
    }
    pub fn CommonNode<T>(
        first: T,
        second: T,
        parent: quest_hook::libil2cpp::Gc<crate::System::Func_2<T, T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CommonNode", (first, second, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn Define(
        &mut self,
        block: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Define", (block))?;
        Ok(__cordl_ret.into())
    }
    pub fn DefinedIn(
        &mut self,
        scope: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DefinedIn", (scope))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureLabel(
        &mut self,
        compiler: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LightCompiler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureLabel", (compiler))?;
        Ok(__cordl_ret.into())
    }
    pub fn FirstDefinition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        > = __cordl_object.invoke("FirstDefinition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLabel(
        &mut self,
        compiler: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LightCompiler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::BranchLabel,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::BranchLabel,
        > = __cordl_object.invoke("GetLabel", (compiler))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node))?;
        Ok(__cordl_object.into())
    }
    pub fn Reference(
        &mut self,
        block: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reference", (block))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateJump(
        &mut self,
        reference: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateJump", (reference))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasDefinitions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDefinitions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasMultipleDefinitions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasMultipleDefinitions", ())?;
        Ok(__cordl_ret.into())
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
