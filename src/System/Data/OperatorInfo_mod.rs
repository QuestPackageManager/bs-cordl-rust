#[cfg(feature = "System+Data+OperatorInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct OperatorInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _type: crate::System::Data::Nodes,
    pub _op: i32,
    pub _priority: i32,
}
#[cfg(feature = "System+Data+OperatorInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::OperatorInfo => "System.Data"
    ."OperatorInfo"
);
#[cfg(feature = "System+Data+OperatorInfo")]
impl std::ops::Deref for crate::System::Data::OperatorInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+OperatorInfo")]
impl std::ops::DerefMut for crate::System::Data::OperatorInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+OperatorInfo")]
impl crate::System::Data::OperatorInfo {
    pub fn New(
        _cordl_type: crate::System::Data::Nodes,
        op: i32,
        pri: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, op, pri))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::System::Data::Nodes,
        op: i32,
        pri: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, op, pri))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+OperatorInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::OperatorInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
