#[cfg(feature = "System+Linq+Expressions+DebugInfoExpression")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugInfoExpression {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Expression,
    >,
    pub _Document_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::SymbolDocumentInfo,
    >,
}
#[cfg(feature = "System+Linq+Expressions+DebugInfoExpression")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::DebugInfoExpression
    => "System.Linq.Expressions"."DebugInfoExpression"
);
#[cfg(feature = "System+Linq+Expressions+DebugInfoExpression")]
impl std::ops::Deref for crate::System::Linq::Expressions::DebugInfoExpression {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Expression,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+DebugInfoExpression")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::DebugInfoExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+DebugInfoExpression")]
impl crate::System::Linq::Expressions::DebugInfoExpression {
    pub fn get_Document(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::SymbolDocumentInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::SymbolDocumentInfo,
        > = __cordl_object.invoke("get_Document", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EndLine(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_EndLine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsClear(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsClear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StartLine(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_StartLine", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+DebugInfoExpression")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::DebugInfoExpression {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
