#[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState+Entry")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct XHashtableState_XHashtable_1_Entry<TValue: quest_hook::libil2cpp::Type> {
    pub Value: TValue,
    pub HashCode: i32,
    pub Next: i32,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState+Entry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Linq::XHashtableState_XHashtable_1_Entry < TValue > =>
    "System.Xml.Linq"."XHashtable`1/XHashtableState/Entry<TValue>" < TValue >
);
#[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState+Entry")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Linq::XHashtableState_XHashtable_1_Entry<TValue> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState+Entry")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::System::Xml::Linq::XHashtableState_XHashtable_1_Entry<TValue> {}
#[cfg(feature = "System+Xml+Linq+XHashtable_1")]
#[repr(C)]
#[derive(Debug)]
pub struct XHashtable_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _state: quest_hook::libil2cpp::Gc<TValue>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::XHashtable_1 < TValue > =>
    "System.Xml.Linq"."XHashtable`1" < TValue >
);
#[cfg(feature = "System+Xml+Linq+XHashtable_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Xml::Linq::XHashtable_1<TValue> {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Xml::Linq::XHashtable_1<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::System::Xml::Linq::XHashtable_1<TValue> {
    #[cfg(feature = "System+Xml+Linq+XHashtable_1+ExtractKeyDelegate")]
    pub type ExtractKeyDelegate = crate::System::Xml::Linq::XHashtable_1_ExtractKeyDelegate<
        TValue,
    >;
    #[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState")]
    pub type XHashtableState = crate::System::Xml::Linq::XHashtable_1_XHashtableState<
        TValue,
    >;
    pub fn Add(&mut self, value: TValue) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("Add", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        extractKey: quest_hook::libil2cpp::Gc<TValue>,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (extractKey, capacity))?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetValue(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        count: i32,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetValue", (key, index, count, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        extractKey: quest_hook::libil2cpp::Gc<TValue>,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (extractKey, capacity))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Linq::XHashtable_1<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+ExtractKeyDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct XHashtable_1_ExtractKeyDelegate<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+ExtractKeyDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Linq::XHashtable_1_ExtractKeyDelegate < TValue > => "System.Xml.Linq"
    ."XHashtable`1/ExtractKeyDelegate" < TValue >
);
#[cfg(feature = "System+Xml+Linq+XHashtable_1+ExtractKeyDelegate")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Xml::Linq::XHashtable_1_ExtractKeyDelegate<TValue> {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+ExtractKeyDelegate")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Xml::Linq::XHashtable_1_ExtractKeyDelegate<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+ExtractKeyDelegate")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::System::Xml::Linq::XHashtable_1_ExtractKeyDelegate<TValue> {
    pub fn Invoke(
        &mut self,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("Invoke", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+ExtractKeyDelegate")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Linq::XHashtable_1_ExtractKeyDelegate<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState")]
#[repr(C)]
#[derive(Debug)]
pub struct XHashtable_1_XHashtableState<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _buckets: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub _entries: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::System::Xml::Linq::XHashtableState_XHashtable_1_Entry<TValue>,
        >,
    >,
    pub _numEntries: i32,
    pub _extractKey: quest_hook::libil2cpp::Gc<TValue>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Linq::XHashtable_1_XHashtableState
    < TValue > => "System.Xml.Linq"."XHashtable`1/XHashtableState" < TValue >
);
#[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Xml::Linq::XHashtable_1_XHashtableState<TValue> {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Xml::Linq::XHashtable_1_XHashtableState<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::System::Xml::Linq::XHashtable_1_XHashtableState<TValue> {
    #[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState+Entry")]
    pub type Entry = crate::System::Xml::Linq::XHashtableState_XHashtable_1_Entry<
        TValue,
    >;
    pub fn ComputeHashCode(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeHashCode", (key, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindEntry(
        &mut self,
        hashCode: i32,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        count: i32,
        entryIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FindEntry", (hashCode, key, index, count, entryIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        extractKey: quest_hook::libil2cpp::Gc<TValue>,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (extractKey, capacity))?;
        Ok(__cordl_object.into())
    }
    pub fn Resize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TValue>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TValue> = __cordl_object
            .invoke("Resize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryAdd(
        &mut self,
        value: TValue,
        newValue: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryAdd", (value, newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        count: i32,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetValue", (key, index, count, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        extractKey: quest_hook::libil2cpp::Gc<TValue>,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (extractKey, capacity))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Linq+XHashtable_1+XHashtableState")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Linq::XHashtable_1_XHashtableState<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
