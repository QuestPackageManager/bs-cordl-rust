#[cfg(feature = "System+Xml+Schema+SequenceNode")]
#[repr(C)]
#[derive(Debug)]
pub struct SequenceNode {
    __cordl_parent: crate::System::Xml::Schema::InteriorNode,
}
#[cfg(feature = "System+Xml+Schema+SequenceNode")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::SequenceNode {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "SequenceNode";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode")]
impl std::ops::Deref for crate::System::Xml::Schema::SequenceNode {
    type Target = crate::System::Xml::Schema::InteriorNode;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode")]
impl std::ops::DerefMut for crate::System::Xml::Schema::SequenceNode {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::BitSet,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::BitSet,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Xml::Schema::BitSet,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ConstructPos")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConstructPos", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (firstpos, lastpos, followpos))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::InteriorNode,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::SymbolsDictionary,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::Positions,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ExpandTree")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExpandTree", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (parent, symbols, positions))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNullable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsNullable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsNullable", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SequenceNode_SequenceConstructPosContext {
    pub this_: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SequenceNode>,
    pub firstpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub lastpos: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub lastposLeft: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
    pub firstposRight: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
}
#[cfg(feature = "System+Xml+Schema+SequenceNode+SequenceConstructPosContext")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::SequenceNode_SequenceConstructPosContext {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "SequenceNode/SequenceConstructPosContext";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode+SequenceConstructPosContext")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Xml::Schema::SequenceNode_SequenceConstructPosContext {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode+SequenceConstructPosContext")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::SequenceNode_SequenceConstructPosContext {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode+SequenceConstructPosContext")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Xml::Schema::SequenceNode_SequenceConstructPosContext {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Xml+Schema+SequenceNode+SequenceConstructPosContext")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Xml::Schema::SequenceNode_SequenceConstructPosContext {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::SequenceNode,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::Schema::BitSet,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::BitSet>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (node, firstpos, lastpos))?
        };
        Ok(__cordl_ret.into())
    }
}
