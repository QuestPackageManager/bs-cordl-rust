#[cfg(feature = "PathsHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct PathsHolder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bezierPath: *mut crate::GlobalNamespace::BezierPath,
    pub _vertexPath: *mut crate::GlobalNamespace::VertexPath,
}
#[cfg(feature = "PathsHolder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PathsHolder => ""."PathsHolder"
);
#[cfg(feature = "PathsHolder")]
impl std::ops::Deref for crate::GlobalNamespace::PathsHolder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PathsHolder")]
impl std::ops::DerefMut for crate::GlobalNamespace::PathsHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PathsHolder")]
impl crate::GlobalNamespace::PathsHolder {
    pub fn New(
        numberOfFixedVertexPathSegments: i32,
        updateVertexPath: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (numberOfFixedVertexPathSegments, updateVertexPath))?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateVertexPathByBezierPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVertexPathByBezierPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        numberOfFixedVertexPathSegments: i32,
        updateVertexPath: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (numberOfFixedVertexPathSegments, updateVertexPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bezierPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BezierPath>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BezierPath> = __cordl_object
            .invoke("get_bezierPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath> = __cordl_object
            .invoke("get_vertexPath", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PathsHolder")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PathsHolder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
