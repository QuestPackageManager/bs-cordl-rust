#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InstructionArray {
    pub MaxStackDepth: i32,
    pub MaxContinuationDepth: i32,
    pub Instructions: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Linq::Expressions::Interpreter::Instruction,
    >,
    pub Objects: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub Labels: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Linq::Expressions::Interpreter::RuntimeLabel,
    >,
    pub DebugCookies: *mut crate::System::Collections::Generic::List_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            i32,
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::InstructionArray =>
    "System.Linq.Expressions.Interpreter"."InstructionArray"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Linq::Expressions::Interpreter::InstructionArray {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray")]
impl crate::System::Linq::Expressions::Interpreter::InstructionArray {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray+DebugView")]
    pub type DebugView = crate::System::Linq::Expressions::Interpreter::InstructionArray_DebugView;
    pub fn _ctor(
        &mut self,
        maxStackDepth: i32,
        maxContinuationDepth: i32,
        instructions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Interpreter::Instruction,
            >,
        >,
        objects: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        labels: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Linq::Expressions::Interpreter::RuntimeLabel,
            >,
        >,
        debugCookies: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    i32,
                    *mut quest_hook::libil2cpp::Il2CppObject,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                maxStackDepth,
                maxContinuationDepth,
                instructions,
                objects,
                labels,
                debugCookies,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray+DebugView")]
#[repr(C)]
#[derive(Debug)]
pub struct InstructionArray_DebugView {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _array: crate::System::Linq::Expressions::Interpreter::InstructionArray,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray+DebugView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::InstructionArray_DebugView =>
    "System.Linq.Expressions.Interpreter"."InstructionArray/DebugView"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray+DebugView")]
impl std::ops::Deref
for crate::System::Linq::Expressions::Interpreter::InstructionArray_DebugView {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray+DebugView")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::InstructionArray_DebugView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray+DebugView")]
impl crate::System::Linq::Expressions::Interpreter::InstructionArray_DebugView {
    pub fn GetInstructionViews(
        &mut self,
        includeDebugCookies: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Linq::Expressions::Interpreter::DebugView_InstructionList_InstructionView,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Linq::Expressions::Interpreter::DebugView_InstructionList_InstructionView,
            >,
        > = __cordl_object.invoke("GetInstructionViews", (includeDebugCookies))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        array: crate::System::Linq::Expressions::Interpreter::InstructionArray,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (array))?;
        Ok(__cordl_object.into())
    }
    pub fn _GetInstructionViews_b__4_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("<GetInstructionViews>b__4_0", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        array: crate::System::Linq::Expressions::Interpreter::InstructionArray,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (array))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InstructionArray+DebugView")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::InstructionArray_DebugView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
