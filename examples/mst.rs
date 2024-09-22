use algograph::{
    graph::*,
    tagged::{self, *},
};
use std::cell::RefCell;
use tagged_ufs::*;

fn main() {
    let vertices = [
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ];
    let dist_fn =
        |a: &(i32, i32), b: &(i32, i32)| (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1);
    let all_edges = {
        let mut res = vec![];
        let mut iter0 = vertices.iter();
        while let Some(v0) = iter0.next() {
            for v1 in iter0.clone() {
                res.push((*v0, *v1));
            }
        }
        res.sort_by_key(|(a, b)| dist_fn(a, b));
        res
    };
    let mst = {
        let mut mst = MinSpanTree::new();
        for v in vertices.iter() {
            mst.overwrite_vertex(*v);
        }
        RefCell::new(mst)
    };
    let mut ufs = UnionFindSets::new();
    for v in vertices.iter() {
        ufs.make_set(
            *v,
            MstTag {
                tree: &mst,
                dist_fn: &dist_fn,
            },
        )
        .unwrap();
    }
    for (v0, v1) in all_edges.iter() {
        ufs.unite(v0, v1).unwrap();
    }
    let mst = mst.into_inner();
    println!("MST:");
    println!("{:?}", mst.debug_with_indent(2, 2));
    println!();
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct E {
    source: VertexId,
    sink: VertexId,
    distance: i32,
}

impl tagged::Edge for E {
    fn source(&self) -> VertexId {
        self.source
    }

    fn sink(&self) -> VertexId {
        self.sink
    }
}

type MinSpanTree = NaiveTaggedGraph<(i32, i32), E, undirected::TreeBackedGraph>;

struct MstTag<'a, DistFn>
where
    DistFn: for<'b> Fn(&'b (i32, i32), &'b (i32, i32)) -> i32,
{
    tree: &'a RefCell<NaiveTaggedGraph<(i32, i32), E, undirected::TreeBackedGraph>>,
    dist_fn: &'a DistFn,
}

impl<'a, DistFn> Mergable<(i32, i32)> for MstTag<'a, DistFn>
where
    DistFn: for<'b> Fn(&'b (i32, i32), &'b (i32, i32)) -> i32,
{
    fn merge<K1, K2>(&mut self, _: Self, key1: &K1, key2: &K2)
    where
        K1: std::borrow::Borrow<(i32, i32)>,
        K2: std::borrow::Borrow<(i32, i32)>,
    {
        let mut tree = self.tree.borrow_mut();
        let vid0 = tree.id_by_vertex(key1.borrow()).unwrap();
        let vid1 = tree.id_by_vertex(key2.borrow()).unwrap();
        let e = E {
            source: vid0,
            sink: vid1,
            distance: (self.dist_fn)(key1.borrow(), key2.borrow()),
        };
        tree.add_edge(e);
    }
}

impl<'a, DistFn> Lengthed for MstTag<'a, DistFn>
where
    DistFn: for<'b> Fn(&'b (i32, i32), &'b (i32, i32)) -> i32,
{
    fn len(&self) -> usize {
        self.tree.borrow().vertex_size()
    }
}
