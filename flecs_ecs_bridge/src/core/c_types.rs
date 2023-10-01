use std::{ffi::CStr, ops::Deref};

use super::c_binding::bindings::*;
use lazy_static::lazy_static;

pub const RUST_ECS_ID_FLAGS_MASK: u64 = 0xFF << 60;
pub const RUST_ECS_COMPONENT_MASK: u64 = !RUST_ECS_ID_FLAGS_MASK;

pub type WorldT = ecs_world_t;
pub type WorldInfoT = ecs_world_info_t;
pub type QueryGroupInfoT = ecs_query_group_info_t;
pub type IdT = ecs_id_t;
pub type EntityT = ecs_entity_t;
pub type TypeT = ecs_type_t;
pub type TableT = ecs_table_t;
pub type FilterT = ecs_filter_t;
pub type ObserverT = ecs_observer_t;
pub type QueryT = ecs_query_t;
pub type RuleT = ecs_rule_t;
pub type RefT = ecs_ref_t;
pub type IterT = ecs_iter_t;
pub type TypeInfoT = ecs_type_info_t;
pub type TypeHooksT = ecs_type_hooks_t;
pub type Flags32T = ecs_flags32_t;

pub static SEPARATOR: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"::\0") };

#[repr(C)]
pub enum InOutKindT {
    InOutDefault = 0,
    InOutNone,
    InOut,
    In,
    Out,
}

//TODO: this is a test
impl InOutKindT {
    pub fn is_read_only(&self) -> bool {
        matches!(self, Self::In)
    }
}

#[repr(C)]
pub enum OperKindT {
    And = 0,
    Or,
    Not,
    Optional,
    AndFrom,
    OrFrom,
    NotFrom,
}

//TODO: this is a test
impl OperKindT {
    pub fn is_negation(&self) -> bool {
        matches!(self, Self::Not | Self::NotFrom)
    }
}

// Id flags
pub const ECS_PAIR: u64 = 1 << 63;
pub const ECS_OVERRIDE: u64 = 1 << 62;
pub const ECS_TOGGLE: u64 = 1 << 61;
pub const ECS_AND: u64 = 1 << 60;

// Builtin component ids
pub const ECS_COMPONENT: u64 = 1;
pub const ECS_IDENTIFIER: u64 = 2;
pub const ECS_ITERABLE: u64 = 3;
pub const ECS_POLY: u64 = 4;

// Poly target components
pub const ECS_QUERY: u64 = 5;
pub const ECS_OBSERVER: u64 = 6;
pub const ECS_SYSTEM: u64 = 7;

// The base ID, equivalent to the C #define
pub const FLECS_HI_COMPONENT_ID: u64 = 256;

// Core scopes & entities
pub const ECS_WORLD: u64 = FLECS_HI_COMPONENT_ID + 0;
pub const ECS_FLECS: u64 = FLECS_HI_COMPONENT_ID + 1;
pub const ECS_FLECS_CORE: u64 = FLECS_HI_COMPONENT_ID + 2;
pub const ECS_FLECS_INTERNALS: u64 = FLECS_HI_COMPONENT_ID + 3;
pub const ECS_MODULE: u64 = FLECS_HI_COMPONENT_ID + 4;
pub const ECS_PRIVATE: u64 = FLECS_HI_COMPONENT_ID + 5;
pub const ECS_PREFAB: u64 = FLECS_HI_COMPONENT_ID + 6;
pub const ECS_DISABLED: u64 = FLECS_HI_COMPONENT_ID + 7;
pub const ECS_SLOT_OF: u64 = FLECS_HI_COMPONENT_ID + 8;
pub const ECS_FLAG: u64 = FLECS_HI_COMPONENT_ID + 9;

// Relationship properties
pub const ECS_WILDCARD: u64 = FLECS_HI_COMPONENT_ID + 10;
pub const ECS_ANY: u64 = FLECS_HI_COMPONENT_ID + 11;
pub const ECS_THIS: u64 = FLECS_HI_COMPONENT_ID + 12;
pub const ECS_VARIABLE: u64 = FLECS_HI_COMPONENT_ID + 13;
pub const ECS_TRANSITIVE: u64 = FLECS_HI_COMPONENT_ID + 14;
pub const ECS_REFLEXIVE: u64 = FLECS_HI_COMPONENT_ID + 15;
pub const ECS_SYMMETRIC: u64 = FLECS_HI_COMPONENT_ID + 16;
pub const ECS_FINAL: u64 = FLECS_HI_COMPONENT_ID + 17;
pub const ECS_DONT_INHERIT: u64 = FLECS_HI_COMPONENT_ID + 18;
pub const ECS_ALWAYS_OVERRIDE: u64 = FLECS_HI_COMPONENT_ID + 19;
pub const ECS_TAG: u64 = FLECS_HI_COMPONENT_ID + 20;
pub const ECS_UNION: u64 = FLECS_HI_COMPONENT_ID + 21;
pub const ECS_EXCLUSIVE: u64 = FLECS_HI_COMPONENT_ID + 22;
pub const ECS_ACYCLIC: u64 = FLECS_HI_COMPONENT_ID + 23;
pub const ECS_TRAVERSABLE: u64 = FLECS_HI_COMPONENT_ID + 24;
pub const ECS_WITH: u64 = FLECS_HI_COMPONENT_ID + 25;
pub const ECS_ONE_OF: u64 = FLECS_HI_COMPONENT_ID + 26;

// Builtin relationships
pub const ECS_CHILD_OF: u64 = FLECS_HI_COMPONENT_ID + 27;
pub const ECS_IS_A: u64 = FLECS_HI_COMPONENT_ID + 28;
pub const ECS_DEPENDS_ON: u64 = FLECS_HI_COMPONENT_ID + 29;

// Identifier tags
pub const ECS_NAME: u64 = FLECS_HI_COMPONENT_ID + 30;
pub const ECS_SYMBOL: u64 = FLECS_HI_COMPONENT_ID + 31;
pub const ECS_ALIAS: u64 = FLECS_HI_COMPONENT_ID + 32;

// Events
pub const ECS_ON_ADD: u64 = FLECS_HI_COMPONENT_ID + 33;
pub const ECS_ON_REMOVE: u64 = FLECS_HI_COMPONENT_ID + 34;
pub const ECS_ON_SET: u64 = FLECS_HI_COMPONENT_ID + 35;
pub const ECS_UNSET: u64 = FLECS_HI_COMPONENT_ID + 36;
pub const ECS_ON_DELETE: u64 = FLECS_HI_COMPONENT_ID + 37;
pub const ECS_ON_TABLE_CREATE: u64 = FLECS_HI_COMPONENT_ID + 38;
pub const ECS_ON_TABLE_DELETE: u64 = FLECS_HI_COMPONENT_ID + 39;
pub const ECS_ON_TABLE_EMPTY: u64 = FLECS_HI_COMPONENT_ID + 40;
pub const ECS_ON_TABLE_FILL: u64 = FLECS_HI_COMPONENT_ID + 41;
pub const ECS_ON_CREATE_TRIGGER: u64 = FLECS_HI_COMPONENT_ID + 42;
pub const ECS_ON_DELETE_TRIGGER: u64 = FLECS_HI_COMPONENT_ID + 43;
pub const ECS_ON_DELETE_OBSERVABLE: u64 = FLECS_HI_COMPONENT_ID + 44;
pub const ECS_ON_COMPONENT_HOOKS: u64 = FLECS_HI_COMPONENT_ID + 45;
pub const ECS_ON_DELETE_TARGET: u64 = FLECS_HI_COMPONENT_ID + 46;
