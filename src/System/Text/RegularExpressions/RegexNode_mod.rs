#[cfg(feature = "System+Text+RegularExpressions+RegexNode")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub NType: i32,
    pub Children: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    >,
    pub Str: *mut quest_hook::libil2cpp::Il2CppString,
    pub Ch: char,
    pub M: i32,
    pub N: i32,
    pub Options: crate::System::Text::RegularExpressions::RegexOptions,
    pub Next: *mut crate::System::Text::RegularExpressions::RegexNode,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexNode =>
    "System.Text.RegularExpressions"."RegexNode"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexNode")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexNode")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexNode")]
impl crate::System::Text::RegularExpressions::RegexNode {
    pub fn AddChild(
        &mut self,
        newChild: *mut crate::System::Text::RegularExpressions::RegexNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChild", (newChild))?;
        Ok(__cordl_ret)
    }
    pub fn Child(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("Child", (i))?;
        Ok(__cordl_ret)
    }
    pub fn ChildCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ChildCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn MakeQuantifier(
        &mut self,
        lazy: bool,
        min: i32,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("MakeQuantifier", (lazy, min, max))?;
        Ok(__cordl_ret)
    }
    pub fn MakeRep(
        &mut self,
        _cordl_type: i32,
        min: i32,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MakeRep", (_cordl_type, min, max))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppString2(
        _cordl_type: i32,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        str: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, options, str))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_char1(
        _cordl_type: i32,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, options, ch))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_3(
        _cordl_type: i32,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        m: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, options, m))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_RegexOptions0(
        _cordl_type: i32,
        options: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, options))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_4(
        _cordl_type: i32,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        m: i32,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, options, m, n))?;
        Ok(__cordl_object)
    }
    pub fn Reduce(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("Reduce", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReduceAlternation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ReduceAlternation", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReduceConcatenation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ReduceConcatenation", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReduceGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ReduceGroup", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReduceRep(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ReduceRep", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReduceSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ReduceSet", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReverseLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ReverseLeft", ())?;
        Ok(__cordl_ret)
    }
    pub fn StripEnation(
        &mut self,
        emptyType: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("StripEnation", (emptyType))?;
        Ok(__cordl_ret)
    }
    pub fn Type(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn UseOptionR(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UseOptionR", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString2(
        &mut self,
        _cordl_type: i32,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        str: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, options, str))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_char1(
        &mut self,
        _cordl_type: i32,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, options, ch))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_3(
        &mut self,
        _cordl_type: i32,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        m: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, options, m))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_RegexOptions0(
        &mut self,
        _cordl_type: i32,
        options: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, options))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_4(
        &mut self,
        _cordl_type: i32,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        m: i32,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, options, m, n))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
