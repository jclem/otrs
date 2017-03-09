use rustc_serialize::json::{Json, Array};

#[derive(Debug)]
pub struct Insert {
    pub pos: usize,
    pub value: Json
}

#[derive(Debug)]
pub struct Delete {
    pub pos: usize
}

#[derive(Debug)]
pub enum Op {
    Insert(Insert),
    Delete(Delete),
    NoOp
}

#[derive(Debug, PartialEq)]
pub enum Side {
    Left,
    Right
}

pub fn insert(value: Json, pos: usize, list: &mut Array) {
    list.insert(pos, value);
}

pub fn delete(pos: usize, list: &mut Array) {
    list.remove(pos);
}

pub fn transform(op_a: Op, op_b: Op, side: Side) -> (Op, Op) {
    match (op_a, op_b) {
        (Op::Insert(ins_a), Op::Insert(ins_b)) =>
            transform_ins_ins(ins_a, ins_b, side),
        (Op::Insert(ins_a), Op::Delete(del_b)) =>
            transform_ins_del(ins_a, del_b),
        (Op::Delete(del_a), Op::Insert(ins_b)) =>
            transform_del_ins(del_a, ins_b),
        (Op::Delete(del_a), Op::Delete(del_b)) =>
            transform_del_del(del_a, del_b),
        (op_a, op_b) =>
            (op_a, op_b)
    }
}

fn transform_ins_ins(ins_a: Insert, ins_b: Insert, side: Side) -> (Op, Op) {
    let mut ins_a = ins_a;
    let mut ins_b = ins_b;

    if ins_a.pos < ins_b.pos {
        ins_b.pos = ins_b.pos + 1;
    } else if ins_a.pos > ins_b.pos {
        ins_a.pos = ins_a.pos + 1;
    } else if side == Side::Left {
        ins_b.pos = ins_b.pos + 1;
    } else {
        ins_a.pos = ins_a.pos + 1;
    }

    return (Op::Insert(ins_a), Op::Insert(ins_b));
}

fn transform_ins_del(ins_a: Insert, del_b: Delete) -> (Op, Op) {
    let mut ins_a = ins_a;
    let mut del_b = del_b;

    if ins_a.pos < del_b.pos {
        del_b.pos = del_b.pos + 1;
    } else if ins_a.pos > del_b.pos {
        ins_a.pos = ins_a.pos - 1;
    } else {
        del_b.pos = del_b.pos + 1;
    }

    return (Op::Insert(ins_a), Op::Delete(del_b));
}

fn transform_del_ins(del_a: Delete, ins_b: Insert) -> (Op, Op) {
    let mut del_a = del_a;
    let mut ins_b = ins_b;

    if del_a.pos < ins_b.pos {
        ins_b.pos = ins_b.pos - 1;
    } else if del_a.pos > ins_b.pos {
        del_a.pos = del_a.pos + 1;
    } else {
        del_a.pos = del_a.pos + 1;
    }

    return (Op::Delete(del_a), Op::Insert(ins_b));
}

fn transform_del_del(del_a: Delete, del_b: Delete) -> (Op, Op) {
    let mut del_a = del_a;
    let mut del_b = del_b;

    if del_a.pos < del_b.pos {
        del_b.pos = del_b.pos - 1;
    } else if del_a.pos > del_b.pos {
        del_a.pos = del_a.pos - 1;
    } else {
        return (Op::NoOp, Op::NoOp)
    }

    return (Op::Delete(del_a), Op::Delete(del_b));
}
