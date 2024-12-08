#[cfg(feature = "System+Data+LikeNode")]
#[repr(C)]
#[derive(Debug)]
pub struct LikeNode {
    __cordl_parent: crate::System::Data::BinaryNode,
    pub _kind: i32,
    pub _pattern: *mut crate::System::String,
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
    pub fn _ctor(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        op: i32,
        left: *mut crate::System::Data::ExpressionNode,
        right: *mut crate::System::Data::ExpressionNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, op, left, right))?;
        Ok(__cordl_ret)
    }
    pub fn Eval(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Eval", (row, version))?;
        Ok(__cordl_ret)
    }
    pub fn AnalyzePattern(
        &mut self,
        pat: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("AnalyzePattern", (pat))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        table: *mut crate::System::Data::DataTable,
        op: i32,
        left: *mut crate::System::Data::ExpressionNode,
        right: *mut crate::System::Data::ExpressionNode,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, op, left, right))?;
        Ok(__cordl_object)
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
