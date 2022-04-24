use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

use derivative::*;
use lazy_static::lazy_static;

use crate::paren_x64;

pub type AlocSet = HashSet<Aloc>;
pub type Assignments<Loc> = HashMap<Aloc, Loc>;
pub type PcAddr = usize;

lazy_static! {
    static ref FVAR_INDEX: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    static ref ALOC_INDEX: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    static ref LABEL_INDEX: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    static ref CURRENT_ASSIGNABLE_REGISTERS: Arc<Mutex<HashSet<Reg>>> =
        Arc::new(Mutex::new(
            vec![
                Reg::rax,
                Reg::rbx,
                Reg::rcx,
                Reg::rdx,
                Reg::rsi,
                Reg::rdi,
                Reg::r8,
                Reg::r9,
                Reg::r12,
                Reg::r13,
                Reg::r14,
                Reg::r15,
            ]
            .into_iter()
            .collect()
        ));
}

fn fresh_index(asbtract_index: &Arc<Mutex<usize>>) -> usize {
    let mut abstract_index = asbtract_index.lock().unwrap();
    let index = *abstract_index;
    *abstract_index += 1;

    index
}

#[cfg(test)]
fn reset_index(asbtract_index: &Arc<Mutex<usize>>) {
    let mut abstract_index = asbtract_index.lock().unwrap();
    *abstract_index = 0usize;
}

#[cfg(test)]
pub fn reset_all_indices() {
    reset_index(&ALOC_INDEX);
    reset_index(&FVAR_INDEX);
    reset_index(&LABEL_INDEX);
}

#[derive(Derivative, Clone, Hash, PartialEq, Eq)]
#[derivative(PartialOrd, Ord)]
pub struct Aloc {
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    pub name: String,

    pub index: usize,
}

impl Aloc {
    pub fn fresh() -> Self {
        let default_name = "tmp";
        let index = fresh_index(&ALOC_INDEX);

        Self {
            name: default_name.into(),
            index,
        }
    }

    pub fn fresh_with_name<I>(name: I) -> Self
    where
        I: Into<String>,
    {
        let index = fresh_index(&ALOC_INDEX);

        Self {
            name: name.into(),
            index,
        }
    }
}

impl Debug for Aloc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.name, self.index))
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Reg {
    rsp,
    rbp,
    rax,
    rbx,
    rcx,
    rdx,
    rsi,
    rdi,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,
}

impl Reg {
    pub fn current_return_reg() -> Self {
        Self::rax
    }

    pub fn current_frame_base_pointer() -> Self {
        Self::rbp
    }

    pub fn current_auxiliary_registers() -> (Self, Self) {
        (Self::r10, Self::r11)
    }

    pub fn set_current_assignable_registers(regs: HashSet<Self>) {
        let mut curr_regs = CURRENT_ASSIGNABLE_REGISTERS.lock().unwrap();
        *curr_regs = regs;
    }

    pub fn current_assignable_registers() -> HashSet<Self> {
        CURRENT_ASSIGNABLE_REGISTERS.lock().unwrap().clone()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Binop {
    plus,
    multiply,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Relop {
    gt,
    gte,
    lt,
    lte,
    eq,
    neq,
}

impl std::ops::Not for Relop {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::gt => Self::lte,
            Self::gte => Self::lt,
            Self::lt => Self::gte,
            Self::lte => Self::gt,
            Self::neq => Self::eq,
            Self::eq => Self::neq,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Label {
    pub label: String,
}

impl Label {
    pub fn new() -> Self {
        let index = fresh_index(&LABEL_INDEX);
        let label = format!("L.tmp.{}", index);

        Self { label }
    }

    pub fn new_with_name<I>(name: I) -> Self
    where
        String: From<I>,
    {
        let index = fresh_index(&LABEL_INDEX);
        let name = String::from(name);
        let label = format!("L.{}.{}", name, index);

        Self { label }
    }

    pub fn halt_label() -> Self {
        let label = format!("L.done");
        Self { label }
    }
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fvar {
    pub index: usize,
}

impl Fvar {
    pub fn fresh() -> Self {
        let index = fresh_index(&FVAR_INDEX);
        Self { index }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Info<Loc> {
    pub locals: Option<AlocSet>,
    pub assignment: Option<Assignments<Loc>>,
    pub undead_out: Option<Node>,
    pub conflicts: Option<Graph>,
}

impl<Loc> Default for Info<Loc> {
    fn default() -> Self {
        Self {
            locals: None,
            assignment: None,
            undead_out: None,
            conflicts: None,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Node {
    alocs { alocs: AlocSet },
    tree { tree: Tree },
}

impl Node {
    pub fn to_alocs_panic(&self) -> &AlocSet {
        match self {
            Self::alocs { alocs } => alocs,
            Self::tree { .. } => {
                panic!("Expected an AlocSet, got a Tree instead...")
            },
        }
    }

    pub fn to_tree_panic(&self) -> &Vec<Self> {
        match self {
            Self::alocs { .. } => {
                panic!("Expected a Tree, got an AlocSet instead...")
            },
            Self::tree {
                tree: Tree { nodes },
            } => nodes,
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::alocs { alocs } => alocs.fmt(f),
            Self::tree { tree } => tree.fmt(f),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Tree {
    pub nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn push_on(&mut self, node: Node) {
        self.nodes.insert(0, node);
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.nodes.clone()).finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    pub graph: HashMap<Aloc, AlocSet>,
}

impl Graph {
    pub fn new(alocs: &AlocSet) -> Self {
        let graph = alocs
            .into_iter()
            .map(|aloc| (aloc.clone(), HashSet::default()))
            .collect();

        Self { graph }
    }

    pub fn insert_alocs(&mut self, aloc: Aloc, alocs: AlocSet) {
        self.graph.insert(aloc, alocs);
    }

    pub fn remove_node(&mut self, aloc: &Aloc) {
        let Self { graph } = self;

        graph.remove(aloc);
        graph.into_iter().for_each(|(_, alocs)| {
            alocs.remove(aloc);
        });
    }

    #[cfg(test)]
    pub fn new_with_graph(graph: &[(Aloc, &[Aloc])]) -> Self {
        let graph = graph.into_iter().fold(
            HashMap::default(),
            |mut graph, (aloc, alocs)| {
                let aloc_set =
                    alocs.into_iter().map(Aloc::clone).collect::<HashSet<_>>();
                graph.insert(aloc.clone(), aloc_set);
                graph
            },
        );

        Self { graph }
    }
}

pub trait Check: Sized {
    fn check(self) -> Result<Self, String>;
}

pub trait Interpret {
    type Output;

    fn interpret(self) -> Self::Output;
}

pub trait Compile {
    fn compile(self, opt_level: crate::OptLevels) -> paren_x64::ParenX64;
}
