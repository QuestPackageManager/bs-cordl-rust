#[cfg(feature = "System+Data+LikeNode")]
#[repr(C)]
#[derive(Debug)]
pub struct LikeNode {
    __cordl_parent: crate::System::Data::BinaryNode,
    pub _kind: i32,
    pub _pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Data+LikeNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::LikeNode => "System.Data"
    ."LikeNode"
);
#[cfg(feature = "System+Data+LikeNode")]
impl std::ops::Deref for crate::System::Data::LikeNode {
    type Target = crate::System::Data::BinaryNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+LikeNode")]
impl std::ops::DerefMut for crate::System::Data::LikeNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+LikeNode")]
impl crate::System::Data::LikeNode {
    pub fn AnalyzePattern(
        &mut self,
        pat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("AnalyzePattern", (pat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Eval(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Eval", (row, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        op: i32,
        left: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
        right: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, op, left, right))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        op: i32,
        left: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
        right: quest_hook::libil2cpp::Gc<crate::System::Data::ExpressionNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, op, left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+LikeNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::LikeNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
