#[cfg(feature = "System+Xml+Schema+SequenceNode")]
#[repr(C)]
#[derive(Debug)]
pub struct SequenceNode {
    __cordl_parent: crate::System::Xml::Schema::InteriorNode,
}
#[cfg(feature = "System+Xml+Schema+SequenceNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::SequenceNode =>
    "System.Xml.Schema"."SequenceNode"
);
#[cfg(feature = "System+Xml+Schema+SequenceNode")]
impl std::ops::Deref for crate::System::Xml::Schema::SequenceNode {
    type Target = crate::System::Xml::Schema::InteriorNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode")]
impl std::ops::DerefMut for crate::System::Xml::Schema::SequenceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode")]
impl crate::System::Xml::Schema::SequenceNode {
    #[cfg(feature = "System+Xml+Schema+SequenceNode+SequenceConstructPosContext")]
    pub type SequenceConstructPosContext = crate::System::Xml::Schema::SequenceNode_SequenceConstructPosContext;
    pub fn ConstructPos(
        &mut self,
        firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        lastpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        followpos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConstructPos", (firstpos, lastpos, followpos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandTree(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::InteriorNode>,
        symbols: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::SymbolsDictionary,
        >,
        positions: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Positions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandTree", (parent, symbols, positions))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNullable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNullable", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::SequenceNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode+SequenceConstructPosContext")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SequenceNode_SequenceConstructPosContext {
    pub this_: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SequenceNode>,
    pub firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub lastpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub lastposLeft: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub firstposRight: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
}
#[cfg(feature = "System+Xml+Schema+SequenceNode+SequenceConstructPosContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::SequenceNode_SequenceConstructPosContext =>
    "System.Xml.Schema"."SequenceNode/SequenceConstructPosContext"
);
#[cfg(feature = "System+Xml+Schema+SequenceNode+SequenceConstructPosContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::SequenceNode_SequenceConstructPosContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode+SequenceConstructPosContext")]
impl crate::System::Xml::Schema::SequenceNode_SequenceConstructPosContext {
    pub fn _ctor(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SequenceNode>,
        firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
        lastpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (node, firstpos, lastpos),
        )?;
        Ok(__cordl_ret.into())
    }
}
