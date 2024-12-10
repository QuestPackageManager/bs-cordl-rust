#[cfg(feature = "System+Xml+Schema+LeafRangeNode")]
#[repr(C)]
#[derive(Debug)]
pub struct LeafRangeNode {
    __cordl_parent: crate::System::Xml::Schema::LeafNode,
    pub min: crate::System::Decimal,
    pub max: crate::System::Decimal,
    pub nextIteration: *mut crate::System::Xml::Schema::BitSet,
}
#[cfg(feature = "System+Xml+Schema+LeafRangeNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::LeafRangeNode =>
    "System.Xml.Schema"."LeafRangeNode"
);
#[cfg(feature = "System+Xml+Schema+LeafRangeNode")]
impl std::ops::Deref for crate::System::Xml::Schema::LeafRangeNode {
    type Target = crate::System::Xml::Schema::LeafNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+LeafRangeNode")]
impl std::ops::DerefMut for crate::System::Xml::Schema::LeafRangeNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+LeafRangeNode")]
impl crate::System::Xml::Schema::LeafRangeNode {
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
    pub fn New_Decimal0(
        min: crate::System::Decimal,
        max: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (min, max))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_Decimal1(
        pos: i32,
        min: crate::System::Decimal,
        max: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pos, min, max))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Decimal0(
        &mut self,
        min: crate::System::Decimal,
        max: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_Decimal1(
        &mut self,
        pos: i32,
        min: crate::System::Decimal,
        max: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pos, min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsRangeNode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsRangeNode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Max(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object.invoke("get_Max", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Min(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object.invoke("get_Min", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NextIteration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet> = __cordl_object
            .invoke("get_NextIteration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_NextIteration(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NextIteration", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+LeafRangeNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::LeafRangeNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
