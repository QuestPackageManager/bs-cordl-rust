#[cfg(feature = "System+Runtime+Serialization+MemberHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberHolder {
    __cordl_parent: crate::System::Object,
    pub _memberType: *mut crate::System::Type,
    pub _context: crate::System::Runtime::Serialization::StreamingContext,
}
#[cfg(feature = "System+Runtime+Serialization+MemberHolder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Serialization::MemberHolder =>
    "System.Runtime.Serialization"."MemberHolder"
);
#[cfg(feature = "System+Runtime+Serialization+MemberHolder")]
impl std::ops::Deref for crate::System::Runtime::Serialization::MemberHolder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+MemberHolder")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::MemberHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+MemberHolder")]
impl crate::System::Runtime::Serialization::MemberHolder {
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: *mut crate::System::Type,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, ctx))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, ctx))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+MemberHolder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::MemberHolder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}