extern crate ot;
extern crate rustc_serialize;

use rustc_serialize::json::Json;
use ot::list::{Insert, Delete, Op};
use ot::list::Side::{Left, Right};

#[test]
fn insert() {
    let mut list = Json::from_str("[1, 3, 4]").unwrap().into_array().unwrap();
    let expected =
        Json::from_str("[1, 2, 3, 4]").unwrap().into_array().unwrap();
    ot::list::insert(Json::U64(2), 1, &mut list);
    assert_eq!(expected, list);
}

#[test]
fn delete() {
    let mut list = Json::from_str("[1, 3, 4]").unwrap().into_array().unwrap();
    let expected = Json::from_str("[1, 4]").unwrap().into_array().unwrap();
    ot::list::delete(1, &mut list);
    assert_eq!(expected, list);
}

#[test]
fn transform_insert_insert_eq_left() {
    let op_a = Op::Insert(Insert { pos: 0, value: Json::U64(1) } );
    let op_b = Op::Insert(Insert { pos: 0, value: Json::U64(2) } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Insert(comp) => assert_eq!(comp.pos, 0),
        _ => unreachable!()
    };

    match op_b {
        Op::Insert(comp) => assert_eq!(comp.pos, 1),
        _ => unreachable!()
    };
}

#[test]
fn transform_insert_insert_eq_right() {
    let op_a = Op::Insert(Insert { pos: 0, value: Json::U64(1) } );
    let op_b = Op::Insert(Insert { pos: 0, value: Json::U64(2) } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Right);

    match op_a {
        Op::Insert(ins) => assert_eq!(ins.pos, 1),
        _ => unreachable!()
    };

    match op_b {
        Op::Insert(ins) => assert_eq!(ins.pos, 0),
        _ => unreachable!()
    };
}

#[test]
fn transform_insert_insert_a_b() {
    let op_a = Op::Insert(Insert { pos: 0, value: Json::U64(1) } );
    let op_b = Op::Insert(Insert { pos: 1, value: Json::U64(2) } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Insert(ins) => assert_eq!(ins.pos, 0),
        _ => unreachable!()
    };

    match op_b {
        Op::Insert(ins) => assert_eq!(ins.pos, 2),
        _ => unreachable!()
    };
}

#[test]
fn transform_insert_insert_b_a() {
    let op_a = Op::Insert(Insert { pos: 1, value: Json::U64(1) } );
    let op_b = Op::Insert(Insert { pos: 0, value: Json::U64(2) } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Insert(ins) => assert_eq!(ins.pos, 2),
        _ => unreachable!()
    };

    match op_b {
        Op::Insert(ins) => assert_eq!(ins.pos, 0),
        _ => unreachable!()
    };
}

#[test]
fn transform_insert_delete_eq() {
    let op_a = Op::Insert(Insert { pos: 0, value: Json::U64(1) } );
    let op_b = Op::Delete(Delete { pos: 0 } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Insert(ins) => assert_eq!(ins.pos, 0),
        _ => unreachable!()
    };

    match op_b {
        Op::Delete(del) => assert_eq!(del.pos, 1),
        _ => unreachable!()
    };
}

#[test]
fn transform_insert_delete_a_b() {
    let op_a = Op::Insert(Insert { pos: 0, value: Json::U64(1) } );
    let op_b = Op::Delete(Delete { pos: 1 } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Insert(ins) => assert_eq!(ins.pos, 0),
        _ => unreachable!()
    };

    match op_b {
        Op::Delete(del) => assert_eq!(del.pos, 2),
        _ => unreachable!()
    };
}

#[test]
fn transform_insert_delete_b_a() {
    let op_a = Op::Insert(Insert { pos: 1, value: Json::U64(1) } );
    let op_b = Op::Delete(Delete { pos: 0 } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Insert(ins) => assert_eq!(ins.pos, 0),
        _ => unreachable!()
    };

    match op_b {
        Op::Delete(del) => assert_eq!(del.pos, 0),
        _ => unreachable!()
    };
}

#[test]
fn transform_delete_insert_eq() {
    let op_a = Op::Delete(Delete { pos: 0 } );
    let op_b = Op::Insert(Insert { pos: 0, value: Json::U64(1) } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Delete(del) => assert_eq!(del.pos, 1),
        _ => unreachable!()
    };

    match op_b {
        Op::Insert(ins) => assert_eq!(ins.pos, 0),
        _ => unreachable!()
    };
}

#[test]
fn transform_delete_insert_a_b() {
    let op_a = Op::Delete(Delete { pos: 0 } );
    let op_b = Op::Insert(Insert { pos: 1, value: Json::U64(1) } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Delete(del) => assert_eq!(del.pos, 0),
        _ => unreachable!()
    };

    match op_b {
        Op::Insert(ins) => assert_eq!(ins.pos, 0),
        _ => unreachable!()
    };
}

#[test]
fn transform_delete_insert_b_a() {
    let op_a = Op::Delete(Delete { pos: 1 } );
    let op_b = Op::Insert(Insert { pos: 0, value: Json::U64(1) } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Delete(del) => assert_eq!(del.pos, 2),
        _ => unreachable!()
    };

    match op_b {
        Op::Insert(ins) => assert_eq!(ins.pos, 0),
        _ => unreachable!()
    };
}

#[test]
fn transform_delete_delete_eq() {
    let op_a = Op::Delete(Delete { pos: 0 } );
    let op_b = Op::Delete(Delete { pos: 0 } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::NoOp => true,
        _ => unreachable!()
    };

    match op_b {
        Op::NoOp => true,
        _ => unreachable!()
    };
}

#[test]
fn transform_delete_delete_a_b() {
    let op_a = Op::Delete(Delete { pos: 0 } );
    let op_b = Op::Delete(Delete { pos: 1 } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Delete(del) => assert_eq!(del.pos, 0),
        _ => unreachable!()
    };

    match op_b {
        Op::Delete(del) => assert_eq!(del.pos, 0),
        _ => unreachable!()
    };
}

#[test]
fn transform_delete_delete_b_a() {
    let op_a = Op::Delete(Delete { pos: 1 } );
    let op_b = Op::Delete(Delete { pos: 0 } );

    let (op_a, op_b) = ot::list::transform(op_a, op_b, Left);

    match op_a {
        Op::Delete(del) => assert_eq!(del.pos, 0),
        _ => unreachable!()
    };

    match op_b {
        Op::Delete(del) => assert_eq!(del.pos, 0),
        _ => unreachable!()
    };
}
