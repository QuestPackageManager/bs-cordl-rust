#[cfg(feature = "IndexFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct IndexFilter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _random: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
    pub _seed: i32,
    pub _groupSize: i32,
    pub _chunkSize: i32,
    pub _visibleCount: i32,
    pub _limitAlsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    pub _start: i32,
    pub _step: i32,
    pub _count: i32,
}
#[cfg(feature = "IndexFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IndexFilter => ""."IndexFilter"
);
#[cfg(feature = "IndexFilter")]
impl std::ops::Deref for crate::GlobalNamespace::IndexFilter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IndexFilter")]
impl std::ops::DerefMut for crate::GlobalNamespace::IndexFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IndexFilter")]
impl crate::GlobalNamespace::IndexFilter {
    #[cfg(feature = "IndexFilter+IndexFilterLimitAlsoAffectType")]
    pub type IndexFilterLimitAlsoAffectType = crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType;
    #[cfg(feature = "IndexFilter+IndexFilterRandomType")]
    pub type IndexFilterRandomType = crate::GlobalNamespace::IndexFilter_IndexFilterRandomType;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::System::ValueTuple_3<i32, i32, i32>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::System::ValueTuple_3<i32, i32, i32>,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        > = __cordl_object.invoke("GetValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_IndexFilter_IndexFilterRandomType_i32_f32_IndexFilter_IndexFilterLimitAlsoAffectType1(
        start: i32,
        end: i32,
        groupSize: i32,
        random: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
        seed: i32,
        chunkSize: i32,
        limit: f32,
        limitAlsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    start,
                    end,
                    groupSize,
                    random,
                    seed,
                    chunkSize,
                    limit,
                    limitAlsoAffectType,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_IndexFilter_IndexFilterRandomType_i32_f32_IndexFilter_IndexFilterLimitAlsoAffectType0(
        start: i32,
        step: i32,
        count: i32,
        groupSize: i32,
        random: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
        seed: i32,
        chunkSize: i32,
        limit: f32,
        limitAlsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    start,
                    step,
                    count,
                    groupSize,
                    random,
                    seed,
                    chunkSize,
                    limit,
                    limitAlsoAffectType,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IndexFilter_IndexFilterRandomType_i32_f32_IndexFilter_IndexFilterLimitAlsoAffectType1(
        &mut self,
        start: i32,
        end: i32,
        groupSize: i32,
        random: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
        seed: i32,
        chunkSize: i32,
        limit: f32,
        limitAlsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    start,
                    end,
                    groupSize,
                    random,
                    seed,
                    chunkSize,
                    limit,
                    limitAlsoAffectType,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_IndexFilter_IndexFilterRandomType_i32_f32_IndexFilter_IndexFilterLimitAlsoAffectType0(
        &mut self,
        start: i32,
        step: i32,
        count: i32,
        groupSize: i32,
        random: crate::GlobalNamespace::IndexFilter_IndexFilterRandomType,
        seed: i32,
        chunkSize: i32,
        limit: f32,
        limitAlsoAffectType: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    start,
                    step,
                    count,
                    groupSize,
                    random,
                    seed,
                    chunkSize,
                    limit,
                    limitAlsoAffectType,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_VisibleCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_VisibleCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_limitAlsoAffectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType = __cordl_object
            .invoke("get_limitAlsoAffectType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_limitsDistribution(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_limitsDistribution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_limitsDuration(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_limitsDuration", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IndexFilter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IndexFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IndexFilter")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    >,
> for crate::GlobalNamespace::IndexFilter {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    >,
> for crate::GlobalNamespace::IndexFilter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter")]
impl AsRef<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    >,
> for crate::GlobalNamespace::IndexFilter {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter")]
impl AsMut<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    >,
> for crate::GlobalNamespace::IndexFilter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::ValueTuple_3<i32, i32, i32>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::GlobalNamespace::IndexFilter {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::GlobalNamespace::IndexFilter {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IndexFilter+IndexFilterLimitAlsoAffectType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFilter_IndexFilterLimitAlsoAffectType {
    Distribution = 2i32,
    Duration = 1i32,
    None = 0i32,
}
#[cfg(feature = "IndexFilter+IndexFilterLimitAlsoAffectType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IndexFilter_IndexFilterLimitAlsoAffectType => ""
    ."IndexFilter/IndexFilterLimitAlsoAffectType"
);
#[cfg(feature = "IndexFilter+IndexFilterRandomType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFilter_IndexFilterRandomType {
    KeepOrder = 1i32,
    NoRandom = 0i32,
    RandomElements = 2i32,
}
#[cfg(feature = "IndexFilter+IndexFilterRandomType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IndexFilter_IndexFilterRandomType => ""
    ."IndexFilter/IndexFilterRandomType"
);
