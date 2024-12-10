#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct LabelScopeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _labels: *mut crate::System::Linq::Expressions::Interpreter::HybridReferenceDictionary_2<
        *mut crate::System::Linq::Expressions::LabelTarget,
        *mut crate::System::Linq::Expressions::Interpreter::LabelInfo,
    >,
    pub Kind: crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
    pub Parent: *mut crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::LabelScopeInfo =>
    "System.Linq.Expressions.Interpreter"."LabelScopeInfo"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::LabelScopeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::LabelScopeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
impl crate::System::Linq::Expressions::Interpreter::LabelScopeInfo {
    pub fn AddLabelInfo(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLabelInfo", (target, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsTarget(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ContainsTarget", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        parent: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        >,
        kind: crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent, kind))?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetLabelInfo(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::LabelTarget>,
        info: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Linq::Expressions::Interpreter::LabelInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetLabelInfo", (target, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::LabelScopeInfo,
        >,
        kind: crate::System::Linq::Expressions::Interpreter::LabelScopeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parent, kind))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanJumpInto(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanJumpInto", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+LabelScopeInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::LabelScopeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
