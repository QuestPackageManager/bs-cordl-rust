#[cfg(feature = "UnityEngine+UI+GridLayoutGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct GridLayoutGroup {
    __cordl_parent: crate::UnityEngine::UI::LayoutGroup,
    pub m_StartCorner: crate::UnityEngine::UI::GridLayoutGroup_Corner,
    pub m_StartAxis: crate::UnityEngine::UI::GridLayoutGroup_Axis,
    pub m_CellSize: crate::UnityEngine::Vector2,
    pub m_Spacing: crate::UnityEngine::Vector2,
    pub m_Constraint: crate::UnityEngine::UI::GridLayoutGroup_Constraint,
    pub m_ConstraintCount: i32,
}
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::GridLayoutGroup =>
    "UnityEngine.UI"."GridLayoutGroup"
);
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup")]
impl std::ops::Deref for crate::UnityEngine::UI::GridLayoutGroup {
    type Target = crate::UnityEngine::UI::LayoutGroup;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup")]
impl std::ops::DerefMut for crate::UnityEngine::UI::GridLayoutGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup")]
impl crate::UnityEngine::UI::GridLayoutGroup {
    #[cfg(feature = "UnityEngine+UI+GridLayoutGroup+Axis")]
    pub type Axis = crate::UnityEngine::UI::GridLayoutGroup_Axis;
    #[cfg(feature = "UnityEngine+UI+GridLayoutGroup+Constraint")]
    pub type Constraint = crate::UnityEngine::UI::GridLayoutGroup_Constraint;
    #[cfg(feature = "UnityEngine+UI+GridLayoutGroup+Corner")]
    pub type Corner = crate::UnityEngine::UI::GridLayoutGroup_Corner;
    pub fn CalculateLayoutInputHorizontal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateLayoutInputHorizontal", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalculateLayoutInputVertical(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateLayoutInputVertical", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetCellsAlongAxis(
        &mut self,
        axis: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCellsAlongAxis", (axis))?;
        Ok(__cordl_ret)
    }
    pub fn SetLayoutHorizontal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutHorizontal", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLayoutVertical(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutVertical", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cellSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_cellSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_constraint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::GridLayoutGroup_Constraint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::GridLayoutGroup_Constraint = __cordl_object
            .invoke("get_constraint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_constraintCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_constraintCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_spacing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_spacing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_startAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::GridLayoutGroup_Axis> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::GridLayoutGroup_Axis = __cordl_object
            .invoke("get_startAxis", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_startCorner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::GridLayoutGroup_Corner> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::GridLayoutGroup_Corner = __cordl_object
            .invoke("get_startCorner", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_cellSize(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cellSize", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_constraint(
        &mut self,
        value: crate::UnityEngine::UI::GridLayoutGroup_Constraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_constraint", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_constraintCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_constraintCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_spacing(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_spacing", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_startAxis(
        &mut self,
        value: crate::UnityEngine::UI::GridLayoutGroup_Axis,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startAxis", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_startCorner(
        &mut self,
        value: crate::UnityEngine::UI::GridLayoutGroup_Corner,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_startCorner", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::GridLayoutGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup+Axis")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridLayoutGroup_Axis {
    Horizontal = 0i32,
    Vertical = 1i32,
}
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup+Axis")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::GridLayoutGroup_Axis =>
    "UnityEngine.UI"."GridLayoutGroup/Axis"
);
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup+Constraint")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridLayoutGroup_Constraint {
    FixedColumnCount = 1i32,
    FixedRowCount = 2i32,
    Flexible = 0i32,
}
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup+Constraint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::GridLayoutGroup_Constraint =>
    "UnityEngine.UI"."GridLayoutGroup/Constraint"
);
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup+Corner")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridLayoutGroup_Corner {
    LowerLeft = 2i32,
    LowerRight = 3i32,
    UpperLeft = 0i32,
    UpperRight = 1i32,
}
#[cfg(feature = "UnityEngine+UI+GridLayoutGroup+Corner")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::GridLayoutGroup_Corner =>
    "UnityEngine.UI"."GridLayoutGroup/Corner"
);
