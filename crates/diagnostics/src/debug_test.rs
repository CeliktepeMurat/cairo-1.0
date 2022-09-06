use std::fmt::Debug;

use db_utils::define_short_id;
use diagnostics_proc_macros::DebugWithDb;

use crate::debug::DebugWithDb;

// Test database query group.
#[salsa::query_group(TestDatabase)]
trait TestGroup {
    #[salsa::interned]
    fn intern_b(&self, crt: DummyLongId) -> DummyShortId;
}
// Database impl.
#[salsa::database(TestDatabase)]
#[derive(Default)]
pub struct DatabaseImpl {
    storage: salsa::Storage<DatabaseImpl>,
}
impl salsa::Database for DatabaseImpl {}

// Structs.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct DummyLongId(usize);

define_short_id!(DummyShortId);
impl DebugWithDb<dyn TestGroup> for DummyShortId {
    fn fmt<'me, 'db>(
        &'me self,
        f: &'me mut std::fmt::Formatter<'_>,
        db: &'db dyn TestGroup,
    ) -> std::fmt::Result {
        db.lookup_intern_b(*self).fmt(f)
    }
}

#[derive(DebugWithDb)]
#[debug_db(TestGroup)]
struct ComplexStruct {
    a: Option<usize>,
    b: DummyShortId,
}

#[test]
fn test_debug() {
    let db = DatabaseImpl::default();
    let a = ComplexStruct { a: Some(5), b: db.intern_b(DummyLongId(6)) };
    assert_eq!(format!("{:?}", a.debug(&db)), "ComplexStruct { a: Some(5), b: DummyLongId(6) }");
}