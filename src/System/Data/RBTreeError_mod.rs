#[cfg(feature = "System+Data+RBTreeError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RBTreeError {
    #[default]
    AttachedNodeWithZerorbTreeNodeId = 18i32,
    CannotRotateInvalidsuccessorNodeinDelete = 11i32,
    CompareNodeInDataRowTree = 19i32,
    CompareSateliteTreeNodeInDataRowTree = 20i32,
    IndexOutOFRangeinGetNodeByIndex = 13i32,
    InvalidNextSizeInDelete = 7i32,
    InvalidNodeSizeinDelete = 9i32,
    InvalidPageSize = 1i32,
    InvalidStateinDelete = 8i32,
    InvalidStateinEndDelete = 10i32,
    InvalidStateinInsert = 5i32,
    NestedSatelliteTreeEnumerator = 21i32,
    NoFreeSlots = 4i32,
    PagePositionInSlotInUse = 3i32,
    RBDeleteFixup = 14i32,
    UnsupportedAccessMethod1 = 15i32,
    UnsupportedAccessMethod2 = 16i32,
    UnsupportedAccessMethodInNonNillRootSubtree = 17i32,
}
#[cfg(feature = "System+Data+RBTreeError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::RBTreeError => "System.Data"
    ."RBTreeError"
);
